/// @SRS{ROS-FUN-BSP-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_rtos() {}

/// @SRS{ROS-FUN-RTOS-020}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_tasklet_persistent_context_data() {}

/// @SRS{ROS-FUN-RTOS-030}
///
/// Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_tasklet_user_data_safe_access() {}

/// @SRS{ROS-FUN-RTOS-040}
///
/// Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_safe_access_to_shared_data() {}

/// @SRS{ROS-FUN-RTOS-6020}
///
/// Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provides_time_with_accuracy_1_ms() {}

/// @SRS{ROS-FUN-RTOS-6030}
///
/// Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provides_time_with_resolution_1_ns() {}

/// @SRS{ROS-FUN-BSP-WDT-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_wdt() {}

/// @SRS{ROS-FUN-BSP-NVIC-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_nvic() {}

/// @SRS{ROS-FUN-BSP-SCB-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_scb() {}

/// @SRS{ROS-FUN-BSP-SCB-060}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_scb_dcache_invalidation() {}

/// @SRS{ROS-FUN-BSP-SCB-070}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_scb_dcache_clean() {}

/// @SRS{ROS-FUN-BSP-SYST-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_syst() {}

/// @SRS{ROS-FUN-BSP-PMC-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_pcm() {}

/// @SRS{ROS-FUN-BSP-UART-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_uart() {}

/// @SRS{ROS-FUN-BSP-PIO-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_pio() {}

/// @SRS{ROS-FUN-BSP-SPI-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_spi() {}

/// @SRS{ROS-FUN-BSP-TIC-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_tic() {}

/// @SRS{ROS-FUN-BSP-XDMAC-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_xdmac() {}

/// @SRS{ROS-FUN-BSP-FPU-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_provide_driver_fpu() {}

/// @SRS{ROS-IF-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_demo_exposes_cnc_interface() {}

/// @SRS{ROS-IF-020}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_demo_uses_ccsds_encapsulation() {}

/// @SRS{ROS-IF-040}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_uart_driver_implements_embedded_hal() {}

/// @SRS{ROS-IF-050}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_pio_driver_implements_embedded_hal() {}

/// @SRS{ROS-IF-060}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_spi_driver_implements_embedded_hal() {}

/// @SRS{ROS-RES-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_bsp_target_samv71() {}

/// @SRS{ROS-RES-020}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_demo_target_samv71() {}

/// @SRS{ROS-DES-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_bsp_implemented_in_rust() {}

/// @SRS{ROS-DES-020}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_implemented_in_rust() {}

/// @SRS{ROS-POR-010}
///
/// Design Analysis is described in N7S-ROS-SVSR-001, chapter 6.2.
#[cfg_attr(not(doc), test)]
fn des_ros_partitioned_into_platform_specific_and_agnostic_code() {}
