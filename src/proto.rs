// This code was autogenerated with `dbus-codegen-rust --file /usr/share/dbus-1/interfaces/org.gnome.ShellSearchProvider2.xml -o src/proto.rs`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgGnomeShellSearchProvider2 {
    fn get_initial_result_set(&self, terms: Vec<&str>) -> Result<Vec<String>, tree::MethodErr>;
    fn get_subsearch_result_set(
        &self,
        previous_results: Vec<&str>,
        terms: Vec<&str>,
    ) -> Result<Vec<String>, tree::MethodErr>;
    fn get_result_metas(
        &self,
        identifiers: Vec<&str>,
    ) -> Result<
        Vec<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
        tree::MethodErr,
    >;
    fn activate_result(
        &self,
        identifier: &str,
        terms: Vec<&str>,
        timestamp: u32,
    ) -> Result<(), tree::MethodErr>;
    fn launch_search(&self, terms: Vec<&str>, timestamp: u32) -> Result<(), tree::MethodErr>;
}

pub fn org_gnome_shell_search_provider2_server<F, T, D>(
    factory: &tree::Factory<tree::MTFn<D>, D>,
    data: D::Interface,
    f: F,
) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgGnomeShellSearchProvider2,
    F: 'static + for<'z> Fn(&'z tree::MethodInfo<tree::MTFn<D>, D>) -> &'z T,
{
    let i = factory.interface("org.gnome.Shell.SearchProvider2", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let terms: Vec<&str> = i.read()?;
        let d = fclone(minfo);
        let results = d.get_initial_result_set(terms)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(results);
        Ok(vec![rm])
    };
    let m = factory.method("GetInitialResultSet", Default::default(), h);
    let m = m.in_arg(("terms", "as"));
    let m = m.out_arg(("results", "as"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let previous_results: Vec<&str> = i.read()?;
        let terms: Vec<&str> = i.read()?;
        let d = fclone(minfo);
        let results = d.get_subsearch_result_set(previous_results, terms)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(results);
        Ok(vec![rm])
    };
    let m = factory.method("GetSubsearchResultSet", Default::default(), h);
    let m = m.in_arg(("previous_results", "as"));
    let m = m.in_arg(("terms", "as"));
    let m = m.out_arg(("results", "as"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let identifiers: Vec<&str> = i.read()?;
        let d = fclone(minfo);
        let metas = d.get_result_metas(identifiers)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(metas);
        Ok(vec![rm])
    };
    let m = factory.method("GetResultMetas", Default::default(), h);
    let m = m.in_arg(("identifiers", "as"));
    let m = m.out_arg(("metas", "aa{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let identifier: &str = i.read()?;
        let terms: Vec<&str> = i.read()?;
        let timestamp: u32 = i.read()?;
        let d = fclone(minfo);
        d.activate_result(identifier, terms, timestamp)?;
        let rm = minfo.msg.method_return();
        Ok(vec![rm])
    };
    let m = factory.method("ActivateResult", Default::default(), h);
    let m = m.in_arg(("identifier", "s"));
    let m = m.in_arg(("terms", "as"));
    let m = m.in_arg(("timestamp", "u"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let terms: Vec<&str> = i.read()?;
        let timestamp: u32 = i.read()?;
        let d = fclone(minfo);
        d.launch_search(terms, timestamp)?;
        let rm = minfo.msg.method_return();
        Ok(vec![rm])
    };
    let m = factory.method("LaunchSearch", Default::default(), h);
    let m = m.in_arg(("terms", "as"));
    let m = m.in_arg(("timestamp", "u"));
    let i = i.add_m(m);

    i
}
