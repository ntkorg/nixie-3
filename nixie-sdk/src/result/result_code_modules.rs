use core::fmt::Display;

use super::modules;

pub static RESULT_CODE_MODULES: [Option<ResultCodeModule>; 512] = [
  None,
  Some(ResultCodeModule { name: "svc", description: "Service Call", get_result_description: modules::svc::get_result_description }),
  Some(ResultCodeModule { name: "fs", description: "Filesystem", get_result_description: modules::fs::get_result_description }),
  Some(ResultCodeModule { name: "os", description: "Operating System", get_result_description: modules::os::get_result_description }),
  Some(ResultCodeModule { name: "htcs", description: "Host Target Connection Sockets", get_result_description: modules::htcs::get_result_description }),
  Some(ResultCodeModule { name: "ncm", description: "Nintendo Content Management", get_result_description: modules::ncm::get_result_description }),
  Some(ResultCodeModule { name: "dd", description: "Device Driver", get_result_description: modules::dd::get_result_description }),
  Some(ResultCodeModule { name: "osdbg", description: "Debug Monitor", get_result_description: modules::osdbg::get_result_description }),
  Some(ResultCodeModule { name: "lr", description: "Location Resolver", get_result_description: modules::lr::get_result_description }),
  Some(ResultCodeModule { name: "ldr", description: "Loader", get_result_description: modules::ldr::get_result_description }),
  Some(ResultCodeModule { name: "sf", description: "CMIF (Inter-Process Communication Command Interface)", get_result_description: modules::sf::get_result_description }),
  Some(ResultCodeModule { name: "sf::hipc", description: "Horizon Inter-Process Communication", get_result_description: modules::hipc::get_result_description }),
  Some(ResultCodeModule { name: "tma", description: "Target Manager Agent - Debugging sysmodule for development kits", get_result_description: modules::tma::get_result_description }),
  Some(ResultCodeModule { name: "dmnt", description: "Target Manager Agent - Debugging sysmodule for development kits", get_result_description: modules::dmnt::get_result_description }),
  Some(ResultCodeModule { name: "gds", description: "(Unknown)", get_result_description: modules::gds::get_result_description }),
  Some(ResultCodeModule { name: "pm", description: "Process Manager", get_result_description: modules::pm::get_result_description }),
  Some(ResultCodeModule { name: "ns", description: "(Unknown)", get_result_description: modules::ns::get_result_description }),
  Some(ResultCodeModule { name: "bsdsockets", description: "BSD Sockets", get_result_description: modules::bsdsockets::get_result_description }),
  Some(ResultCodeModule { name: "htc", description: "Host-Target Connection", get_result_description: modules::htc::get_result_description }),
  Some(ResultCodeModule { name: "tsc", description: "(Unknown)", get_result_description: modules::tsc::get_result_description }),
  Some(ResultCodeModule { name: "kvdb", description: "Key-Value Database", get_result_description: modules::kvdb::get_result_description }),
  Some(ResultCodeModule { name: "sm", description: "Service Manager", get_result_description: modules::sm::get_result_description }),
  Some(ResultCodeModule { name: "ro", description: "Relocatable Object Module", get_result_description: modules::ro::get_result_description }),
  Some(ResultCodeModule { name: "gc", description: "Game Card", get_result_description: modules::gc::get_result_description }),
  Some(ResultCodeModule { name: "sdmmc", description: "Secure Digital Multimedia Card", get_result_description: modules::sdmmc::get_result_description }),
  Some(ResultCodeModule { name: "ovln", description: "(Unknown)", get_result_description: modules::ovln::get_result_description }),
  Some(ResultCodeModule { name: "spl", description: "Secure Platform Services", get_result_description: modules::spl::get_result_description }),
  Some(ResultCodeModule { name: "socket", description: "Socket", get_result_description: modules::socket::get_result_description }),
  None, // Missing 28
  Some(ResultCodeModule { name: "htclow", description: "Host-Target Connection Low-Level", get_result_description: modules::htclow::get_result_description }),
  Some(ResultCodeModule { name: "ddsf", description: "Device Driver Service Framework", get_result_description: modules::ddsf::get_result_description }),
  Some(ResultCodeModule { name: "htcfs", description: "Host-Target Connection File System", get_result_description: modules::htcfs::get_result_description }),
  Some(ResultCodeModule { name: "async", description: "(Unknown)", get_result_description: modules::r#async::get_result_description }),
  Some(ResultCodeModule { name: "util", description: "(Unknown)", get_result_description: modules::util::get_result_description }),
  None, // Missing 34
  Some(ResultCodeModule { name: "tipc", description: "(Unknown)", get_result_description: modules::tipc::get_result_description }),
  None, // Missing 36
  Some(ResultCodeModule { name: "ainf", description: "(Unknown)", get_result_description: modules::ainf::get_result_description }),
  None, // Missing 38
  Some(ResultCodeModule { name: "crt", description: "(Unknown)", get_result_description: modules::crt::get_result_description }),

  None, None, None, None, None, None, None, None, None, None, // 10x (next is 50)
  None, None, None, None, None, None, None, None, None, None, // 10x (next is 60)
  None, None, None, None, None, None, None, None, None, None, // 10x (next is 70)
  None, None, None, None, None, None, None, None, None, None, // 10x (next is 80)
  None, None, None, None, None, None, None, None, None, None, // 10x (next is 90)
  None, None, None, None, None, None, None, None, None, None, // 10x (next is 100)

  Some(ResultCodeModule { name: "eth", description: "Ethernet", get_result_description: modules::eth::get_result_description }),
  Some(ResultCodeModule { name: "i2c", description: "Inter-Integrated Circuit", get_result_description: modules::i2c::get_result_description }),
  Some(ResultCodeModule { name: "gpio", description: "General Purpose I/O", get_result_description: modules::gpio::get_result_description }),
  Some(ResultCodeModule { name: "uart", description: "Universal Asynchronous Receiver / Transmitter", get_result_description: modules::uart::get_result_description }),
  Some(ResultCodeModule { name: "cpad", description: "(Unknown)", get_result_description: modules::cpad::get_result_description }),
  Some(ResultCodeModule { name: "settings", description: "Settings", get_result_description: modules::settings::get_result_description }),
  Some(ResultCodeModule { name: "ftm", description: "(Unknown... lol)", get_result_description: modules::ftm::get_result_description }),
  Some(ResultCodeModule { name: "wlan", description: "Wireless Local Area Network", get_result_description: modules::wlan::get_result_description }),
  Some(ResultCodeModule { name: "xcd", description: "(Unknown)", get_result_description: modules::xcd::get_result_description }),
  Some(ResultCodeModule { name: "tmp451", description: "Temperature Sensor", get_result_description: modules::tmp451::get_result_description }),
  Some(ResultCodeModule { name: "nifm", description: "Network Interface Manager", get_result_description: modules::nifm::get_result_description }),
  Some(ResultCodeModule { name: "codec", description: "(Unknown)", get_result_description: modules::codec::get_result_description }),
  Some(ResultCodeModule { name: "lsm6ds3", description: "Accelerometer & Gyroscope", get_result_description: modules::codec::get_result_description }),
  Some(ResultCodeModule { name: "bluetooth", description: "Bluetooth", get_result_description: modules::codec::get_result_description }),
  Some(ResultCodeModule { name: "vi", description: "Video Interface", get_result_description: modules::vi::get_result_description }),
  Some(ResultCodeModule { name: "nfp", description: "Nintendo Figurine / Amiibo Protocol", get_result_description: modules::nfp::get_result_description }),
  Some(ResultCodeModule { name: "time", description: "System time / clock", get_result_description: modules::time::get_result_description }),
  Some(ResultCodeModule { name: "fgm", description: "(Unknown)", get_result_description: modules::fgm::get_result_description }),
  Some(ResultCodeModule { name: "oe", description: "Operating Environment", get_result_description: modules::oe::get_result_description }),
  Some(ResultCodeModule { name: "bh1730fvc", description: "Ambient Light Sensor", get_result_description: modules::bh1730fvc::get_result_description }),
  Some(ResultCodeModule { name: "pcie", description: "Peripheral Component Interconnect Express", get_result_description: modules::pcie::get_result_description }),
  Some(ResultCodeModule { name: "friends", description: "(Unknown)", get_result_description: modules::friends::get_result_description }),
  Some(ResultCodeModule { name: "bcat", description: "Background Content Asymmetric Synchronized Delivery and Transmission", get_result_description: modules::bcat::get_result_description }),
  Some(ResultCodeModule { name: "ssl", description: "Secure Sockets Layer", get_result_description: modules::bcat::get_result_description }),
  Some(ResultCodeModule { name: "account", description: "(Unknown)", get_result_description: modules::bcat::get_result_description }),
  Some(ResultCodeModule { name: "news", description: "(Unknown)", get_result_description: modules::bcat::get_result_description }),

  // TODO: finish!

  None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
  None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
  None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
  None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None
];

#[derive(Copy, Clone)]
pub struct ResultCodeModule {
  pub name: &'static str,
  pub description: &'static str,
  pub get_result_description: fn (code: u32) -> Option<ResultCodeDescription>,
}

pub struct ResultCodeDescription {
  pub name: &'static str,
  pub description: &'static str,
  pub namespace: Option<&'static str>,
}

impl Display for ResultCodeDescription {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self.namespace {
      None => f.write_str(&self.name),
      Some(namespace) => f.write_fmt(format_args!("{}::{}", namespace, self.name)),
    }
  }
}