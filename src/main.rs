#![allow(deprecated)]
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::Result;
use dbus::arg::Variant;
use dbus::blocking::stdintf::org_freedesktop_dbus::RequestNameReply;
use dbus::tree::MethodErr;

mod conf;
mod proto;

#[derive(Debug, Clone)]
struct PasswordStore {
    base: PathBuf,
}

#[derive(Default)]
struct PasswordStoreType;

impl dbus::tree::DataType for PasswordStoreType {
    type Tree = ();
    type ObjectPath = PasswordStore;
    type Property = ();
    type Interface = ();
    type Method = ();
    type Signal = ();
}

impl PasswordStore {
    fn from_home(mut base: PathBuf) -> Self {
        base.push(".password-store");

        Self { base }
    }

    fn search(&self, terms: &[&str]) -> Vec<String> {
        let dir = walkdir::WalkDir::new(&self.base)
            .into_iter()
            .filter_entry(|e| e.path() == &self.base || !is_hidden(&e.path()));

        let mut res = Vec::new();

        for entry in dir {
            let entry = match entry {
                Ok(e) => e,
                _ => continue,
            };

            let path = match entry.path().strip_prefix(&self.base) {
                Ok(path) => path,
                Err(_) => continue,
            };

            let path = match path.to_str() {
                Some(s) => s,
                None => continue,
            };

            let name = match path.strip_suffix(".gpg") {
                Some(s) => s,
                None => continue,
            };

            if terms.iter().all(|t| name.contains(t)) {
                res.push(path.to_owned());
            }
        }

        res
    }

    fn get_meta_by_id(&self, id: &str) -> Option<HashMap<String, VarRefArg>> {
        let mut meta = HashMap::<String, VarRefArg>::new();

        let path = self.base.join(id);

        match std::fs::metadata(&path) {
            Ok(_) => (),
            Err(_) => return None,
        };

        let path = PathBuf::from(id);
        let stem = match path.file_stem() {
            Some(stem) => stem.to_string_lossy().into_owned(),
            None => return None,
        };
        let desc = match path.parent() {
            Some(par) => par.to_string_lossy().into_owned(),
            None => return None,
        };

        meta.insert(String::from("id"), Variant(Box::new(String::from(id))));
        meta.insert(String::from("name"), Variant(Box::new(stem)));
        meta.insert(String::from("description"), Variant(Box::new(desc)));
        meta.insert(
            String::from("gicon"),
            Variant(Box::new(String::from("dialog-password"))),
        );

        Some(meta)
    }

    fn activate(&self, id: &str) -> Result<()> {
        let name = match id.strip_suffix(".gpg") {
            Some(name) => name,
            None => return Ok(()),
        };

        let conf = conf::load()?;
        let (cmd, args) = conf.copy_cmd.split_first().unwrap();

        let mut child = Command::new(cmd)
            .args(args)
            .arg(&name)
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdout = BufReader::new(child.stdout.take().unwrap());

        let mut out = Vec::new();
        stdout.read_until(b'\n', &mut out)?;

        let out = String::from_utf8_lossy(&out);

        Command::new("notify-send")
            .args(&["-i", "dialog-password", "Pass", &out])
            .output()?;

        // pass normally exits by now, so we shouldn't be blocked
        child.wait()?;

        Ok(())
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false)
}

type MResult<T> = std::result::Result<T, MethodErr>;
type VarRefArg = Variant<Box<dyn dbus::arg::RefArg>>;

impl proto::OrgGnomeShellSearchProvider2 for PasswordStore {
    fn get_initial_result_set(&self, terms: Vec<&str>) -> MResult<Vec<String>> {
        Ok(self.search(&terms))
    }

    fn get_subsearch_result_set(&self, _prev: Vec<&str>, terms: Vec<&str>) -> MResult<Vec<String>> {
        self.get_initial_result_set(terms)
    }

    fn get_result_metas(&self, ids: Vec<&str>) -> MResult<Vec<HashMap<String, VarRefArg>>> {
        Ok(ids
            .into_iter()
            .filter_map(|id| self.get_meta_by_id(id))
            .collect())
    }

    fn activate_result(&self, id: &str, _terms: Vec<&str>, _timestamp: u32) -> MResult<()> {
        if let Err(e) = self.activate(id) {
            eprintln!("activation failed: {}", e);
        }

        Ok(())
    }

    fn launch_search(&self, terms: Vec<&str>, timestamp: u32) -> MResult<()> {
        let it = self.search(&terms);
        let id = match it.first() {
            Some(id) => id,
            None => return Ok(()),
        };

        self.activate_result(id, terms, timestamp)
    }
}

fn main() -> Result<()> {
    let store = match std::env::home_dir() {
        Some(home) => PasswordStore::from_home(home),
        None => anyhow::bail!("could not determine home directory"),
    };

    let conn = dbus::blocking::LocalConnection::new_session()?;

    match conn.request_name("net.setattr.PasswordStoreSearch", false, true, false)? {
        RequestNameReply::PrimaryOwner => (),
        _ => anyhow::bail!("failed to aquire bus name"),
    }

    let factory = dbus::tree::Factory::new_fn::<PasswordStoreType>();
    let iface =
        proto::org_gnome_shell_search_provider2_server(&factory, (), move |m| m.path.get_data());

    let tree = factory.tree(()).add(
        factory
            .object_path("/search", store)
            .introspectable()
            .add(iface),
    );

    tree.start_receive(&conn);

    loop {
        conn.process(Duration::new(1, 0))?;
    }
}
