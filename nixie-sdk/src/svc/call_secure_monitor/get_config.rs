use bitfield_struct::bitfield;

use super::{
  call_secure_monitor,
  SecureMonitorArgument::{Null, Value},
  SmcResult,
};

pub fn get_is_program_verification_disabled() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(1), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub enum DramId {
  EristaIcosaSamsung4gb,
  EristaIcosaHynix4gb,
  EristaIcosaMicron4gb,
  MarikoIowaHynix1y4gb,
  EristaIcosaSamsung6gb,
  MarikoHoagHynix1y4gb,
  MarikoAulaHynix1y4gb,
  MarikoIowaSamsung4gb,
  MarikoIowaSamsung8gb,
  MarikoIowaHynix4gb,
  MarikoIowaMicron4gb,
  MarikoHoagSamsung4gb,
  MarikoHoagSamsung8gb,
  MarikoHoagHynix4gb,
  MarikoHoagMicron4gb,
  MarikoIowaSamsung1y4gbX,
  MarikoIowaSamsung1y8gbX,
  MarikoHoagSamsung1y4gbX,
  MarikoIowaSamsung1z4gb,
  MarikoHoagSamsung1z4gb,
  MarikoAulaSamsung1z4gb,
  MarikoHoagSamsung1y8gbX,
  MarikoAulaSamsung1y4gbX,
  MarikoIowaMicron1y4gb,
  MarikoHoagMicron1y4gb,
  MarikoAulaMicron1y4gb,
  MarikoAulaSamsung1y8gbX,
  MarikoIowaHynix1a4gb,
  MarikoHoagHynix1a4gb,
  MarikoAulaHynix1a4gb,
  MarikoIowaMicron1a4gb,
  MarikoHoagMicron1a4gb,
  MarikoAulaMicron1a4gb,

  Unknown(u64),
}

pub enum DramGeneration {
  LPDDR4,
  LPDDR4X,

  Unknown,
}

pub enum DramManufacturer {
  Samsung,
  Hynix,
  Micron,

  Unknown,
}

impl DramId {
  pub fn get_manufacturer(&self) -> DramManufacturer {
    match self {
      DramId::MarikoAulaHynix1a4gb => DramManufacturer::Hynix,
      DramId::MarikoAulaHynix1y4gb => DramManufacturer::Hynix,
      DramId::MarikoHoagHynix1a4gb => DramManufacturer::Hynix,
      DramId::MarikoHoagHynix1y4gb => DramManufacturer::Hynix,
      DramId::MarikoIowaHynix1a4gb => DramManufacturer::Hynix,
      DramId::MarikoIowaHynix1y4gb => DramManufacturer::Hynix,
      DramId::EristaIcosaHynix4gb => DramManufacturer::Hynix,
      DramId::MarikoHoagHynix4gb => DramManufacturer::Hynix,
      DramId::MarikoIowaHynix4gb => DramManufacturer::Hynix,

      DramId::EristaIcosaSamsung4gb => DramManufacturer::Samsung,
      DramId::EristaIcosaSamsung6gb => DramManufacturer::Samsung,
      DramId::MarikoIowaSamsung4gb => DramManufacturer::Samsung,
      DramId::MarikoIowaSamsung8gb => DramManufacturer::Samsung,
      DramId::MarikoHoagSamsung4gb => DramManufacturer::Samsung,
      DramId::MarikoHoagSamsung8gb => DramManufacturer::Samsung,
      DramId::MarikoIowaSamsung1y4gbX => DramManufacturer::Samsung,
      DramId::MarikoIowaSamsung1y8gbX => DramManufacturer::Samsung,
      DramId::MarikoHoagSamsung1y4gbX => DramManufacturer::Samsung,
      DramId::MarikoIowaSamsung1z4gb => DramManufacturer::Samsung,
      DramId::MarikoHoagSamsung1z4gb => DramManufacturer::Samsung,
      DramId::MarikoAulaSamsung1z4gb => DramManufacturer::Samsung,
      DramId::MarikoHoagSamsung1y8gbX => DramManufacturer::Samsung,
      DramId::MarikoAulaSamsung1y4gbX => DramManufacturer::Samsung,
      DramId::MarikoAulaSamsung1y8gbX => DramManufacturer::Samsung,

      DramId::EristaIcosaMicron4gb => DramManufacturer::Micron,
      DramId::MarikoIowaMicron4gb => DramManufacturer::Micron,
      DramId::MarikoHoagMicron4gb => DramManufacturer::Micron,
      DramId::MarikoIowaMicron1y4gb => DramManufacturer::Micron,
      DramId::MarikoHoagMicron1y4gb => DramManufacturer::Micron,
      DramId::MarikoAulaMicron1y4gb => DramManufacturer::Micron,
      DramId::MarikoIowaMicron1a4gb => DramManufacturer::Micron,
      DramId::MarikoHoagMicron1a4gb => DramManufacturer::Micron,
      DramId::MarikoAulaMicron1a4gb => DramManufacturer::Micron,

      DramId::Unknown(..) => DramManufacturer::Unknown,
    }
  }

  pub fn get_generation(&self) -> DramGeneration {
    match self {
      DramId::MarikoIowaSamsung1y4gbX => DramGeneration::LPDDR4X,
      DramId::MarikoIowaSamsung1y8gbX => DramGeneration::LPDDR4X,
      DramId::MarikoHoagSamsung1y4gbX => DramGeneration::LPDDR4X,
      DramId::MarikoHoagSamsung1y8gbX => DramGeneration::LPDDR4X,
      DramId::MarikoAulaSamsung1y4gbX => DramGeneration::LPDDR4X,
      DramId::MarikoAulaSamsung1y8gbX => DramGeneration::LPDDR4X,

      DramId::EristaIcosaSamsung4gb => DramGeneration::LPDDR4,
      DramId::EristaIcosaHynix4gb => DramGeneration::LPDDR4,
      DramId::EristaIcosaMicron4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaHynix1y4gb => DramGeneration::LPDDR4,
      DramId::EristaIcosaSamsung6gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagHynix1y4gb => DramGeneration::LPDDR4,
      DramId::MarikoAulaHynix1y4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaSamsung4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaSamsung8gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaHynix4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaMicron4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagSamsung4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagSamsung8gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagHynix4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagMicron4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaSamsung1z4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagSamsung1z4gb => DramGeneration::LPDDR4,
      DramId::MarikoAulaSamsung1z4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaMicron1y4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagMicron1y4gb => DramGeneration::LPDDR4,
      DramId::MarikoAulaMicron1y4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaHynix1a4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagHynix1a4gb => DramGeneration::LPDDR4,
      DramId::MarikoAulaHynix1a4gb => DramGeneration::LPDDR4,
      DramId::MarikoIowaMicron1a4gb => DramGeneration::LPDDR4,
      DramId::MarikoHoagMicron1a4gb => DramGeneration::LPDDR4,
      DramId::MarikoAulaMicron1a4gb => DramGeneration::LPDDR4,

      DramId::Unknown(..) => DramGeneration::Unknown,
    }
  }

  pub fn get_capacity_gb(&self) -> Option<u8> {
    match self {
      DramId::EristaIcosaSamsung4gb => Some(4),
      DramId::EristaIcosaHynix4gb => Some(4),
      DramId::EristaIcosaMicron4gb => Some(4),
      DramId::MarikoIowaHynix1y4gb => Some(4),
      DramId::MarikoHoagHynix1y4gb => Some(4),
      DramId::MarikoAulaHynix1y4gb => Some(4),
      DramId::MarikoIowaSamsung4gb => Some(4),
      DramId::MarikoIowaHynix4gb => Some(4),
      DramId::MarikoIowaMicron4gb => Some(4),
      DramId::MarikoHoagSamsung4gb => Some(4),
      DramId::MarikoHoagHynix4gb => Some(4),
      DramId::MarikoHoagMicron4gb => Some(4),
      DramId::MarikoIowaSamsung1y4gbX => Some(4),
      DramId::MarikoHoagSamsung1y4gbX => Some(4),
      DramId::MarikoIowaSamsung1z4gb => Some(4),
      DramId::MarikoHoagSamsung1z4gb => Some(4),
      DramId::MarikoAulaSamsung1z4gb => Some(4),
      DramId::MarikoAulaSamsung1y4gbX => Some(4),
      DramId::MarikoIowaMicron1y4gb => Some(4),
      DramId::MarikoHoagMicron1y4gb => Some(4),
      DramId::MarikoAulaMicron1y4gb => Some(4),
      DramId::MarikoIowaHynix1a4gb => Some(4),
      DramId::MarikoHoagHynix1a4gb => Some(4),
      DramId::MarikoAulaHynix1a4gb => Some(4),
      DramId::MarikoIowaMicron1a4gb => Some(4),
      DramId::MarikoHoagMicron1a4gb => Some(4),
      DramId::MarikoAulaMicron1a4gb => Some(4),

      DramId::EristaIcosaSamsung6gb => Some(6),

      DramId::MarikoIowaSamsung8gb => Some(8),
      DramId::MarikoHoagSamsung8gb => Some(8),
      DramId::MarikoIowaSamsung1y8gbX => Some(8),
      DramId::MarikoHoagSamsung1y8gbX => Some(8),
      DramId::MarikoAulaSamsung1y8gbX => Some(8),

      DramId::Unknown(..) => None,
    }
  }
}

pub fn get_dram_id() -> Result<DramId, SmcResult> {
  call_secure_monitor(2, [Value(2), Null, Null, Null, Null, Null, Null]).map(|v| match v[0] {
    0 => DramId::EristaIcosaSamsung4gb,
    1 => DramId::EristaIcosaHynix4gb,
    2 => DramId::EristaIcosaMicron4gb,
    3 => DramId::MarikoIowaHynix1y4gb,
    4 => DramId::EristaIcosaSamsung6gb,
    5 => DramId::MarikoHoagHynix1y4gb,
    6 => DramId::MarikoAulaHynix1y4gb,

    8 => DramId::MarikoIowaSamsung4gb,
    9 => DramId::MarikoIowaSamsung8gb,
    10 => DramId::MarikoIowaHynix4gb,
    11 => DramId::MarikoIowaMicron4gb,
    12 => DramId::MarikoHoagSamsung4gb,
    13 => DramId::MarikoHoagSamsung8gb,
    14 => DramId::MarikoHoagHynix4gb,
    15 => DramId::MarikoHoagMicron4gb,

    17 => DramId::MarikoIowaSamsung1y4gbX,
    18 => DramId::MarikoIowaSamsung1y8gbX,
    19 => DramId::MarikoHoagSamsung1y4gbX,
    20 => DramId::MarikoIowaSamsung1z4gb,
    21 => DramId::MarikoHoagSamsung1z4gb,
    22 => DramId::MarikoAulaSamsung1z4gb,
    23 => DramId::MarikoHoagSamsung1y8gbX,
    24 => DramId::MarikoAulaSamsung1y4gbX,
    25 => DramId::MarikoIowaMicron1y4gb,
    26 => DramId::MarikoHoagMicron1y4gb,
    27 => DramId::MarikoAulaMicron1y4gb,
    28 => DramId::MarikoAulaSamsung1y8gbX,
    29 => DramId::MarikoIowaHynix1a4gb,
    30 => DramId::MarikoHoagHynix1a4gb,
    31 => DramId::MarikoAulaHynix1a4gb,
    32 => DramId::MarikoIowaMicron1a4gb,
    33 => DramId::MarikoHoagMicron1a4gb,
    34 => DramId::MarikoAulaMicron1a4gb,

    v => DramId::Unknown(v),
  })
}

pub fn get_security_engine_interrupt_handler() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(3), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub fn get_fuse_version() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(4), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub enum HardwareType {
  /// Retail Erista (First Generation) Units, along with Erista EDEVs and SDEVs
  Icosa,
  /// An unreleased Erista model
  Copper,
  /// Retail Mariko (Second Generation) Units, along with Mariko EDEVs and SDEVs
  Hoag,
  /// Retail Mariko Lite (Nintendo Switch Lite) Units, along with HDEVs (Nintendo Switch Lite Devkits)
  Iowa,
  /// An unreleased Mariko model
  Calcio,
  /// Retail Mariko OLED (Nintendo Switch OLED) Units, along with ADEVs (Nintendo Switch OLED Devkits)
  Aula,

  Unknown(u64),
}

pub fn get_hardware_type() -> Result<HardwareType, SmcResult> {
  call_secure_monitor(2, [Value(5), Null, Null, Null, Null, Null, Null]).map(|v| match v[0] {
    0 => HardwareType::Icosa,
    1 => HardwareType::Copper,
    2 => HardwareType::Hoag,
    3 => HardwareType::Iowa,
    4 => HardwareType::Calcio,
    5 => HardwareType::Aula,

    v => HardwareType::Unknown(v),
  })
}

pub enum HardwareState {
  Development,
  Production,
  Unknown(u64),
}

pub fn get_hardware_state() -> Result<HardwareState, SmcResult> {
  call_secure_monitor(2, [Value(6), Null, Null, Null, Null, Null, Null]).map(|v| match v[0] {
    0 => HardwareState::Development,
    1 => HardwareState::Production,

    v => HardwareState::Unknown(v),
  })
}

pub fn get_is_recovery_boot() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(7), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_device_id() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(8), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub enum BootReason {
  /// A.K.A. `AcOk`. Occurs when the switch turns on due to power being available to the device
  PowerAvailable,

  /// A.K.A. `OnKey`. Occurs when the user presses the power button
  PowerButton,

  /// RealTimeClock alarms, allows the switch to power itself on after a certain time interval has passed.
  RtcAlarm1,

  /// RealTimeClock alarms, allows the switch to power itself on after a certain time interval has passed.
  RtcAlarm2,

  Unknown(u64),
}

pub fn get_boot_reason() -> Result<BootReason, SmcResult> {
  call_secure_monitor(2, [Value(9), Null, Null, Null, Null, Null, Null]).map(|v| match v[0] {
    1 => BootReason::PowerAvailable,
    2 => BootReason::PowerButton,
    3 => BootReason::RtcAlarm1,
    4 => BootReason::RtcAlarm2,

    v => BootReason::Unknown(v),
  })
}

#[bitfield(u8)]
pub struct MemoryMode {
  #[bits(4)]
  purpose: u8,
  #[bits(4)]
  size: u8,
}

pub fn get_memory_mode() -> Result<MemoryMode, SmcResult> {
  call_secure_monitor(2, [Value(10), Null, Null, Null, Null, Null, Null])
    .map(|v| MemoryMode::from_bits(v[0] as u8))
}

pub fn get_is_development_function_enabled() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(11), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

#[bitfield(u64)]
pub struct KernelConfiguration {
  enable_non_zero_fill_memory: bool,
  enable_user_exception_handler: bool,
  enable_usermode_pmu_access: bool,
  enable_extra_thread_resource_allocation: bool,
  disable_dynamic_system_resource_allocation: bool,

  #[bits(3)]
  padding_0: u8,

  call_show_error_on_panic: bool,

  #[bits(7)]
  padding_1: u8,

  #[bits(2)]
  memory_size: u8,

  #[bits(46)]
  padding_2: u64,
}

pub fn get_kernel_configuration() -> Result<KernelConfiguration, SmcResult> {
  call_secure_monitor(2, [Value(12), Null, Null, Null, Null, Null, Null])
    .map(|v| KernelConfiguration::from_bits(v[0]))
}

/// Hi-Z (high impedence) mode is a state the charger IC (bq24192) can be in.
///
/// When in Hi-Z mode, the charger is fully disconnected from the circuit, and cannot charge the battery.
///
/// Hi-Z mode being disabled doesn't tell you that the switch is currently charging, only that it's capable of being charged.
pub fn get_is_charger_in_hiz_state() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(13), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_is_in_retail_interactive_display_state() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(14), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_regulator_type() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(15), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub fn get_device_unique_key_generation() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(16), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

// Exosphere Extension Items

pub struct ExosphereApiVersion {
  pub atmosphere_release_version: (u8, u8, u8),
  pub key_generation_version: u8,
  pub target_firmware: (u8, u8, u8),
}

impl ExosphereApiVersion {
  fn from_u64(v: u64) -> ExosphereApiVersion {
    ExosphereApiVersion {
      atmosphere_release_version: ((v >> 56) as u8, (v >> 48) as u8, (v >> 40) as u8),
      key_generation_version: (v >> 40) as u8,
      target_firmware: ((v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8),
    }
  }
}

pub fn get_exosphere_api_version() -> Result<ExosphereApiVersion, SmcResult> {
  call_secure_monitor(2, [Value(65000), Null, Null, Null, Null, Null, Null])
    .map(|v| ExosphereApiVersion::from_u64(v[0]))
}

pub fn get_exosphere_needs_reboot() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65001), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_exosphere_needs_shutdown() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65002), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_exosphere_git_commit_hash() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(65003), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub fn get_exosphere_has_rcm_bug_patch() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65004), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_exosphere_should_simulate_blank_prodinfo() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65005), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_exosphere_should_allow_writes_to_calibration_partition() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65006), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub enum ExosphereMemoryManagementControllerType {
  System,
  SdCardEmulated,

  Unknown(u64),
}

pub fn get_exosphere_memory_management_controller_type(
) -> Result<ExosphereMemoryManagementControllerType, SmcResult> {
  call_secure_monitor(2, [Value(65007), Null, Null, Null, Null, Null, Null]).map(|v| match v[0] {
    0 => ExosphereMemoryManagementControllerType::System,
    1 => ExosphereMemoryManagementControllerType::SdCardEmulated,

    v => ExosphereMemoryManagementControllerType::Unknown(v),
  })
}

pub fn get_exosphere_payload_address() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(65008), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub fn get_exosphere_log_configuration() -> Result<u64, SmcResult> {
  call_secure_monitor(2, [Value(65009), Null, Null, Null, Null, Null, Null]).map(|v| v[0])
}

pub fn get_exosphere_force_enable_usb3() -> Result<bool, SmcResult> {
  call_secure_monitor(2, [Value(65010), Null, Null, Null, Null, Null, Null]).map(|v| v[0] != 0)
}

pub fn get_exosphere_supported_horizon_version() -> Result<(u8, u8, u8), SmcResult> {
  call_secure_monitor(2, [Value(65011), Null, Null, Null, Null, Null, Null])
    .map(|v| ((v[0] >> 24) as u8, (v[0] >> 16) as u8, (v[0] >> 8) as u8))
}

pub fn get_exosphere_approximate_api_version() -> Result<ExosphereApiVersion, SmcResult> {
  call_secure_monitor(2, [Value(65012), Null, Null, Null, Null, Null, Null])
    .map(|v| ExosphereApiVersion::from_u64(v[0]))
}
