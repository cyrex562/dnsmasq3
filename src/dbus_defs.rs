
pub const introspection_xml_template: &str =
"<!DOCTYPE node PUBLIC \"-//freedesktop//DTD D-BUS Object Introspection 1.0//EN\"\n" +
"\"http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd\">\n" +
"<node name=\"" + DNSMASQ_PATH + "\">\n" +
"  <interface name=\"org.freedesktop.DBus.Introspectable\">\n" +
"    <method name=\"Introspect\">\n" +
"      <arg name=\"data\" direction=\"out\" type=\"s\"/>\n" +
"    </method>\n" +
"  </interface>\n" +
"  <interface name=\"%s\">\n" +
"    <method name=\"ClearCache\">\n" +
"    </method>\n" +
"    <method name=\"GetVersion\">\n" +
"      <arg name=\"version\" direction=\"out\" type=\"s\"/>\n" +
"    </method>\n" +
"    <method name=\"GetLoopServers\">\n" +
"      <arg name=\"server\" direction=\"out\" type=\"as\"/>\n" +
"    </method>\n" +
"    <method name=\"SetServers\">\n" +
"      <arg name=\"servers\" direction=\"in\" type=\"av\"/>\n" +
"    </method>\n" +
"    <method name=\"SetDomainServers\">\n" +
"      <arg name=\"servers\" direction=\"in\" type=\"as\"/>\n" +
"    </method>\n" +
"    <method name=\"SetServersEx\">\n" +
"      <arg name=\"servers\" direction=\"in\" type=\"aas\"/>\n" +
"    </method>\n" +
"    <method name=\"SetFilterWin2KOption\">\n" +
"      <arg name=\"filterwin2k\" direction=\"in\" type=\"b\"/>\n" +
"    </method>\n" +
"    <method name=\"SetBogusPrivOption\">\n" +
"      <arg name=\"boguspriv\" direction=\"in\" type=\"b\"/>\n" +
"    </method>\n" +
"    <signal name=\"DhcpLeaseAdded\">\n" +
"      <arg name=\"ipaddr\" type=\"s\"/>\n" +
"      <arg name=\"hwaddr\" type=\"s\"/>\n" +
"      <arg name=\"hostname\" type=\"s\"/>\n" +
"    </signal>\n" +
"    <signal name=\"DhcpLeaseDeleted\">\n" +
"      <arg name=\"ipaddr\" type=\"s\"/>\n" +
"      <arg name=\"hwaddr\" type=\"s\"/>\n" +
"      <arg name=\"hostname\" type=\"s\"/>\n" +
"    </signal>\n" +
"    <signal name=\"DhcpLeaseUpdated\">\n" +
"      <arg name=\"ipaddr\" type=\"s\"/>\n" +
"      <arg name=\"hwaddr\" type=\"s\"/>\n" +
"      <arg name=\"hostname\" type=\"s\"/>\n" +
"    </signal>\n" +
"    <method name=\"AddDhcpLease\">\n" +
"       <arg name=\"ipaddr\" type=\"s\"/>\n" +
"       <arg name=\"hwaddr\" type=\"s\"/>\n" +
"       <arg name=\"hostname\" type=\"ay\"/>\n" +
"       <arg name=\"clid\" type=\"ay\"/>\n" +
"       <arg name=\"lease_duration\" type=\"u\"/>\n" +
"       <arg name=\"ia_id\" type=\"u\"/>\n" +
"       <arg name=\"is_temporary\" type=\"b\"/>\n" +
"    </method>\n" +
"    <method name=\"DeleteDhcpLease\">\n" +
"       <arg name=\"ipaddr\" type=\"s\"/>\n" +
"       <arg name=\"success\" type=\"b\" direction=\"out\"/>\n" +
"    </method>\n" +
"    <method name=\"GetMetrics\">\n" +
"      <arg name=\"metrics\" direction=\"out\" type=\"a{su}\"/>\n" +
"    </method>\n" +
"  </interface>\n" +
"</node>\n";

// TODO: replace global string
// static char *introspection_xml = nullptr;

pub struct DBusWatch {
  refcount: i32,
  fd: DBusPollable,
  flags: u32,
  handler: DBusWatchHandler,
  handler_data: c_void,
  free_handler_data_function: DBusFreeFunction,
  data: c_void,
  free_data_function: DBusFreeFunction,
  // enabled: us
  // oom_last
  flags2: u32 // 0: enabled, 1: oom_last_time
}

pub struct watch {
  watch: DBusWatch,
  // next: struct watch
}

type dbus_bool_t = u32;