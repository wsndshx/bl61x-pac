#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    chip_inform: CHIP_INFORM,
    _reserved1: [u8; 0x4c],
    _reserved_1_core_cfg16: [u8; 0x08],
    _reserved_2_core_cfg18: [u8; 0x08],
    _reserved_3_core_cfg20: [u8; 0x08],
    _reserved4: [u8; 0x28],
    clock_config_0: CLOCK_CONFIG_0,
    clock_config_1: CLOCK_CONFIG_1,
    _reserved6: [u8; 0x08],
    bus_config_0: BUS_CONFIG_0,
    _reserved7: [u8; 0x6c],
    gpadc_config: GPADC_CONFIG,
    _reserved8: [u8; 0x0c],
    gpdac_config_0: GPDAC_CONFIG_0,
    gpdac_config_1: GPDAC_CONFIG_1,
    gpdac_config_2: GPDAC_CONFIG_2,
    gpdac_config_3: GPDAC_CONFIG_3,
    dma_config_0: DMA_CONFIG_0,
    dma_config_1: DMA_CONFIG_1,
    dma_config_2: DMA_CONFIG_2,
    _reserved15: [u8; 0x04],
    ir_config_0: IR_CONFIG_0,
    ir_config_1: IR_CONFIG_1,
    _reserved17: [u8; 0x08],
    uart_config: UART_CONFIG,
    uart_signal_0: UART_SIGNAL_0,
    uart_signal_1: UART_SIGNAL_1,
    _reserved20: [u8; 0x14],
    flash_config: FLASH_CONFIG,
    _reserved21: [u8; 0x0c],
    i2c_config: I2C_CONFIG,
    _reserved22: [u8; 0x0c],
    i2s_config: I2S_CONFIG,
    _reserved23: [u8; 0x1c],
    spi_config: SPI_CONFIG,
    _reserved24: [u8; 0x0c],
    pio_cfg0: PIO_CFG0,
    _reserved25: [u8; 0x0c],
    pwm_cfg0: PWM_CFG0,
    _reserved26: [u8; 0x0c],
    pwm_config: PWM_CONFIG,
    _reserved27: [u8; 0x0c],
    dbi_config: DBI_CONFIG,
    _reserved28: [u8; 0x5c],
    digit_clock_0: DIGIT_CLOCK_0,
    digit_clock_1: DIGIT_CLOCK_1,
    digit_clock_2: DIGIT_CLOCK_2,
    _reserved31: [u8; 0x04],
    radio_config: RADIO_CONFIG,
    _reserved32: [u8; 0x7c],
    debug_config_0: DEBUG_CONFIG_0,
    debug_config_1: DEBUG_CONFIG_1,
    debug_config_2: DEBUG_CONFIG_2,
    debug_config_3: DEBUG_CONFIG_3,
    debug_config_4: DEBUG_CONFIG_4,
    _reserved37: [u8; 0x0c],
    self_test_0: SELF_TEST_0,
    self_test_1: SELF_TEST_1,
    _reserved39: [u8; 0x18],
    bmx_cfg0: BMX_CFG0,
    bmx_cfg1: BMX_CFG1,
    bmx_cfg2: BMX_CFG2,
    bmx_cfg3: BMX_CFG3,
    bmx_cfg4: BMX_CFG4,
    bmx_cfg5: BMX_CFG5,
    bmx_cfg6: BMX_CFG6,
    _reserved46: [u8; 0x04],
    audio_config_0: AUDIO_CONFIG_0,
    audio_config_1: AUDIO_CONFIG_1,
    _reserved48: [u8; 0x48],
    emac_config: EMAC_CONFIG,
    _reserved49: [u8; 0x8c],
    cam_cfg0: CAM_CFG0,
    _reserved50: [u8; 0x0c],
    sdh_config: SDH_CONFIG,
    _reserved51: [u8; 0x0c],
    sdio_cfg0: SDIO_CFG0,
    _reserved52: [u8; 0x4c],
    permit_config: PERMIT_CONFIG,
    _reserved53: [u8; 0x7c],
    glb_parm_cfg0: GLB_PARM_CFG0,
    _reserved54: [u8; 0x10],
    debug_cfg1: DEBUG_CFG1,
    _reserved55: [u8; 0x08],
    reset_sts0: RESET_STS0,
    _reserved56: [u8; 0x0c],
    swrst_cfg0: SWRST_CFG0,
    swrst_s1: SWRST_S1,
    swrst_cfg2: SWRST_CFG2,
    swrst_cfg3: SWRST_CFG3,
    _reserved60: [u8; 0x30],
    cgen_m: CGEN_M,
    cgen_cfg1: CGEN_CFG1,
    cgen_cfg2: CGEN_CFG2,
    cgen_cfg3: CGEN_CFG3,
    _reserved64: [u8; 0x70],
    reg_sram_ret: REG_SRAM_RET,
    reg_sram_slp: REG_SRAM_SLP,
    reg_sram_parm: REG_SRAM_PARM,
    sram_cfg3: SRAM_CFG3,
    reg_sram_parm2: REG_SRAM_PARM2,
    _reserved69: [u8; 0x0c],
    psram_config: PSRAM_CONFIG,
    _reserved70: [u8; 0xcc],
    proc_mon: PROC_MON,
    _reserved71: [u8; 0x011c],
    wifi_pll_config_0: WIFI_PLL_CONFIG_0,
    wifi_pll_config_1: WIFI_PLL_CONFIG_1,
    wifi_pll_config_2: WIFI_PLL_CONFIG_2,
    wifi_pll_config_3: WIFI_PLL_CONFIG_3,
    wifi_pll_config_4: WIFI_PLL_CONFIG_4,
    wifi_pll_config_5: WIFI_PLL_CONFIG_5,
    wifi_pll_config_6: WIFI_PLL_CONFIG_6,
    wifi_pll_config_7: WIFI_PLL_CONFIG_7,
    wifi_pll_config_8: WIFI_PLL_CONFIG_8,
    wifi_pll_config_9: WIFI_PLL_CONFIG_9,
    wifi_pll_config_10: WIFI_PLL_CONFIG_10,
    wifi_pll_config_11: WIFI_PLL_CONFIG_11,
    wifi_pll_config_12: WIFI_PLL_CONFIG_12,
    wifi_pll_config_13: WIFI_PLL_CONFIG_13,
    wifi_pll_config_14: WIFI_PLL_CONFIG_14,
    _reserved86: [u8; 0x38],
    ldo18: LDO18,
    _reserved87: [u8; 0x3c],
    gpio_config: [GPIO_CONFIG; 35],
    _reserved88: [u8; 0x0174],
    gpio_input_0: GPIO_INPUT_0,
    gpio_input_1: GPIO_INPUT_1,
    _reserved90: [u8; 0x18],
    gpio_output_0: GPIO_OUTPUT_0,
    gpio_output_1: GPIO_OUTPUT_1,
    gpio_set_0: GPIO_SET_0,
    gpio_set_1: GPIO_SET_1,
    gpio_clear_0: GPIO_CLEAR_0,
    gpio_clear_1: GPIO_CLEAR_1,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip information register"]
    #[inline(always)]
    pub const fn chip_inform(&self) -> &CHIP_INFORM {
        &self.chip_inform
    }
    #[doc = "0x50 - core_cfg16."]
    #[inline(always)]
    pub const fn core_cfg16(&self) -> &CORE_CFG16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50..0x58 - Chip interrupt state register"]
    #[inline(always)]
    pub const fn interrupt_state(&self, n: usize) -> &INTERRUPT_STATE {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(80).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - Chip interrupt state register"]
    #[inline(always)]
    pub fn interrupt_state_iter(&self) -> impl Iterator<Item = &INTERRUPT_STATE> {
        (0..2).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(80).add(4 * n).cast() })
    }
    #[doc = "0x54 - core_cfg17."]
    #[inline(always)]
    pub const fn core_cfg17(&self) -> &CORE_CFG17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - core_cfg18."]
    #[inline(always)]
    pub const fn core_cfg18(&self) -> &CORE_CFG18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58..0x60 - Chip interrupt mask register"]
    #[inline(always)]
    pub const fn interrupt_mask(&self, n: usize) -> &INTERRUPT_MASK {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(88).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x60 - Chip interrupt mask register"]
    #[inline(always)]
    pub fn interrupt_mask_iter(&self) -> impl Iterator<Item = &INTERRUPT_MASK> {
        (0..2).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(88).add(4 * n).cast() })
    }
    #[doc = "0x5c - core_cfg19."]
    #[inline(always)]
    pub const fn core_cfg19(&self) -> &CORE_CFG19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - core_cfg20."]
    #[inline(always)]
    pub const fn core_cfg20(&self) -> &CORE_CFG20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x60..0x68 - Chip clear interrupt register"]
    #[inline(always)]
    pub const fn interrupt_clear(&self, n: usize) -> &INTERRUPT_CLEAR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(96).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x68 - Chip clear interrupt register"]
    #[inline(always)]
    pub fn interrupt_clear_iter(&self) -> impl Iterator<Item = &INTERRUPT_CLEAR> {
        (0..2).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(96).add(4 * n).cast() })
    }
    #[doc = "0x64 - core_cfg21."]
    #[inline(always)]
    pub const fn core_cfg21(&self) -> &CORE_CFG21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x90 - System clock configuration register 0"]
    #[inline(always)]
    pub const fn clock_config_0(&self) -> &CLOCK_CONFIG_0 {
        &self.clock_config_0
    }
    #[doc = "0x94 - System clock configuration register 1"]
    #[inline(always)]
    pub const fn clock_config_1(&self) -> &CLOCK_CONFIG_1 {
        &self.clock_config_1
    }
    #[doc = "0xa0 - Bus configuration register 0"]
    #[inline(always)]
    pub const fn bus_config_0(&self) -> &BUS_CONFIG_0 {
        &self.bus_config_0
    }
    #[doc = "0x110 - General Purpose Analog-to-digital convert configuration"]
    #[inline(always)]
    pub const fn gpadc_config(&self) -> &GPADC_CONFIG {
        &self.gpadc_config
    }
    #[doc = "0x120 - General Purpose Digital-to-analog convert configuration 0"]
    #[inline(always)]
    pub const fn gpdac_config_0(&self) -> &GPDAC_CONFIG_0 {
        &self.gpdac_config_0
    }
    #[doc = "0x124 - General Purpose Digital-to-analog convert configuration 1"]
    #[inline(always)]
    pub const fn gpdac_config_1(&self) -> &GPDAC_CONFIG_1 {
        &self.gpdac_config_1
    }
    #[doc = "0x128 - General Purpose Digital-to-analog convert configuration 2"]
    #[inline(always)]
    pub const fn gpdac_config_2(&self) -> &GPDAC_CONFIG_2 {
        &self.gpdac_config_2
    }
    #[doc = "0x12c - General Purpose Digital-to-analog convert configuration 3"]
    #[inline(always)]
    pub const fn gpdac_config_3(&self) -> &GPDAC_CONFIG_3 {
        &self.gpdac_config_3
    }
    #[doc = "0x130 - Direct Memory Access configuration 0"]
    #[inline(always)]
    pub const fn dma_config_0(&self) -> &DMA_CONFIG_0 {
        &self.dma_config_0
    }
    #[doc = "0x134 - Direct Memory Access configuration 1"]
    #[inline(always)]
    pub const fn dma_config_1(&self) -> &DMA_CONFIG_1 {
        &self.dma_config_1
    }
    #[doc = "0x138 - Direct Memory Access configuration 2"]
    #[inline(always)]
    pub const fn dma_config_2(&self) -> &DMA_CONFIG_2 {
        &self.dma_config_2
    }
    #[doc = "0x140 - Infrared configuration register 0"]
    #[inline(always)]
    pub const fn ir_config_0(&self) -> &IR_CONFIG_0 {
        &self.ir_config_0
    }
    #[doc = "0x144 - Infrared configuration register 1"]
    #[inline(always)]
    pub const fn ir_config_1(&self) -> &IR_CONFIG_1 {
        &self.ir_config_1
    }
    #[doc = "0x150 - Universal Asynchronous Receiver/Transmitter configuration"]
    #[inline(always)]
    pub const fn uart_config(&self) -> &UART_CONFIG {
        &self.uart_config
    }
    #[doc = "0x154 - Universal Asynchronous Receiver/Transmitter signal configuration 0"]
    #[inline(always)]
    pub const fn uart_signal_0(&self) -> &UART_SIGNAL_0 {
        &self.uart_signal_0
    }
    #[doc = "0x158 - Universal Asynchronous Receiver/Transmitter signal configuration 1"]
    #[inline(always)]
    pub const fn uart_signal_1(&self) -> &UART_SIGNAL_1 {
        &self.uart_signal_1
    }
    #[doc = "0x170 - Serial flash configuration"]
    #[inline(always)]
    pub const fn flash_config(&self) -> &FLASH_CONFIG {
        &self.flash_config
    }
    #[doc = "0x180 - Inter-Integrated Circuit bus configuration"]
    #[inline(always)]
    pub const fn i2c_config(&self) -> &I2C_CONFIG {
        &self.i2c_config
    }
    #[doc = "0x190 - Inter-IC Sound configuration"]
    #[inline(always)]
    pub const fn i2s_config(&self) -> &I2S_CONFIG {
        &self.i2s_config
    }
    #[doc = "0x1b0 - Serial Peripheral Interface configuration"]
    #[inline(always)]
    pub const fn spi_config(&self) -> &SPI_CONFIG {
        &self.spi_config
    }
    #[doc = "0x1c0 - pio_cfg0."]
    #[inline(always)]
    pub const fn pio_cfg0(&self) -> &PIO_CFG0 {
        &self.pio_cfg0
    }
    #[doc = "0x1d0 - pwm_cfg0."]
    #[inline(always)]
    pub const fn pwm_cfg0(&self) -> &PWM_CFG0 {
        &self.pwm_cfg0
    }
    #[doc = "0x1e0 - Pulse-Width configuration"]
    #[inline(always)]
    pub const fn pwm_config(&self) -> &PWM_CONFIG {
        &self.pwm_config
    }
    #[doc = "0x1f0 - MIPI Display Bus Interface clock configuration"]
    #[inline(always)]
    pub const fn dbi_config(&self) -> &DBI_CONFIG {
        &self.dbi_config
    }
    #[doc = "0x250 - Digital clock configuration 0"]
    #[inline(always)]
    pub const fn digit_clock_0(&self) -> &DIGIT_CLOCK_0 {
        &self.digit_clock_0
    }
    #[doc = "0x254 - Digital clock configuration 1"]
    #[inline(always)]
    pub const fn digit_clock_1(&self) -> &DIGIT_CLOCK_1 {
        &self.digit_clock_1
    }
    #[doc = "0x258 - Digital clock configuration 2"]
    #[inline(always)]
    pub const fn digit_clock_2(&self) -> &DIGIT_CLOCK_2 {
        &self.digit_clock_2
    }
    #[doc = "0x260 - Radio frequency configuration register"]
    #[inline(always)]
    pub const fn radio_config(&self) -> &RADIO_CONFIG {
        &self.radio_config
    }
    #[doc = "0x2e0 - Debug configuration register 0"]
    #[inline(always)]
    pub const fn debug_config_0(&self) -> &DEBUG_CONFIG_0 {
        &self.debug_config_0
    }
    #[doc = "0x2e4 - Debug configuration register 1"]
    #[inline(always)]
    pub const fn debug_config_1(&self) -> &DEBUG_CONFIG_1 {
        &self.debug_config_1
    }
    #[doc = "0x2e8 - Debug configuration register 2"]
    #[inline(always)]
    pub const fn debug_config_2(&self) -> &DEBUG_CONFIG_2 {
        &self.debug_config_2
    }
    #[doc = "0x2ec - Debug configuration register 3"]
    #[inline(always)]
    pub const fn debug_config_3(&self) -> &DEBUG_CONFIG_3 {
        &self.debug_config_3
    }
    #[doc = "0x2f0 - Debug configuration register 4"]
    #[inline(always)]
    pub const fn debug_config_4(&self) -> &DEBUG_CONFIG_4 {
        &self.debug_config_4
    }
    #[doc = "0x300 - Machine Built-in Self Test register 0"]
    #[inline(always)]
    pub const fn self_test_0(&self) -> &SELF_TEST_0 {
        &self.self_test_0
    }
    #[doc = "0x304 - Machine Built-in Self Test register 1"]
    #[inline(always)]
    pub const fn self_test_1(&self) -> &SELF_TEST_1 {
        &self.self_test_1
    }
    #[doc = "0x320 - bmx_cfg0."]
    #[inline(always)]
    pub const fn bmx_cfg0(&self) -> &BMX_CFG0 {
        &self.bmx_cfg0
    }
    #[doc = "0x324 - bmx_cfg1."]
    #[inline(always)]
    pub const fn bmx_cfg1(&self) -> &BMX_CFG1 {
        &self.bmx_cfg1
    }
    #[doc = "0x328 - bmx_cfg2."]
    #[inline(always)]
    pub const fn bmx_cfg2(&self) -> &BMX_CFG2 {
        &self.bmx_cfg2
    }
    #[doc = "0x32c - bmx_cfg3."]
    #[inline(always)]
    pub const fn bmx_cfg3(&self) -> &BMX_CFG3 {
        &self.bmx_cfg3
    }
    #[doc = "0x330 - bmx_cfg4."]
    #[inline(always)]
    pub const fn bmx_cfg4(&self) -> &BMX_CFG4 {
        &self.bmx_cfg4
    }
    #[doc = "0x334 - bmx_cfg5."]
    #[inline(always)]
    pub const fn bmx_cfg5(&self) -> &BMX_CFG5 {
        &self.bmx_cfg5
    }
    #[doc = "0x338 - bmx_cfg6."]
    #[inline(always)]
    pub const fn bmx_cfg6(&self) -> &BMX_CFG6 {
        &self.bmx_cfg6
    }
    #[doc = "0x340 - Audio configuration register 0"]
    #[inline(always)]
    pub const fn audio_config_0(&self) -> &AUDIO_CONFIG_0 {
        &self.audio_config_0
    }
    #[doc = "0x344 - Audio configuration register 1"]
    #[inline(always)]
    pub const fn audio_config_1(&self) -> &AUDIO_CONFIG_1 {
        &self.audio_config_1
    }
    #[doc = "0x390 - Ethernet Media Access Control configuration"]
    #[inline(always)]
    pub const fn emac_config(&self) -> &EMAC_CONFIG {
        &self.emac_config
    }
    #[doc = "0x420 - cam_cfg0."]
    #[inline(always)]
    pub const fn cam_cfg0(&self) -> &CAM_CFG0 {
        &self.cam_cfg0
    }
    #[doc = "0x430 - Secure Digital host configuration"]
    #[inline(always)]
    pub const fn sdh_config(&self) -> &SDH_CONFIG {
        &self.sdh_config
    }
    #[doc = "0x440 - sdio_cfg0."]
    #[inline(always)]
    pub const fn sdio_cfg0(&self) -> &SDIO_CFG0 {
        &self.sdio_cfg0
    }
    #[doc = "0x490 - Permission control peripheral configuration"]
    #[inline(always)]
    pub const fn permit_config(&self) -> &PERMIT_CONFIG {
        &self.permit_config
    }
    #[doc = "0x510 - glb_parm_cfg0."]
    #[inline(always)]
    pub const fn glb_parm_cfg0(&self) -> &GLB_PARM_CFG0 {
        &self.glb_parm_cfg0
    }
    #[doc = "0x524 - debug_cfg1."]
    #[inline(always)]
    pub const fn debug_cfg1(&self) -> &DEBUG_CFG1 {
        &self.debug_cfg1
    }
    #[doc = "0x530 - reset_sts0."]
    #[inline(always)]
    pub const fn reset_sts0(&self) -> &RESET_STS0 {
        &self.reset_sts0
    }
    #[doc = "0x540 - swrst_s1_ext + swrst_s3 + swrst_s2"]
    #[inline(always)]
    pub const fn swrst_cfg0(&self) -> &SWRST_CFG0 {
        &self.swrst_cfg0
    }
    #[doc = "0x544 - swrst_s1."]
    #[inline(always)]
    pub const fn swrst_s1(&self) -> &SWRST_S1 {
        &self.swrst_s1
    }
    #[doc = "0x548 - swrst_cfg2."]
    #[inline(always)]
    pub const fn swrst_cfg2(&self) -> &SWRST_CFG2 {
        &self.swrst_cfg2
    }
    #[doc = "0x54c - Disable hreset"]
    #[inline(always)]
    pub const fn swrst_cfg3(&self) -> &SWRST_CFG3 {
        &self.swrst_cfg3
    }
    #[doc = "0x580 - cgen_m."]
    #[inline(always)]
    pub const fn cgen_m(&self) -> &CGEN_M {
        &self.cgen_m
    }
    #[doc = "0x584 - cgen_s1a + cgen_s1"]
    #[inline(always)]
    pub const fn cgen_cfg1(&self) -> &CGEN_CFG1 {
        &self.cgen_cfg1
    }
    #[doc = "0x588 - cgen_s1_ext + cgen_s3"]
    #[inline(always)]
    pub const fn cgen_cfg2(&self) -> &CGEN_CFG2 {
        &self.cgen_cfg2
    }
    #[doc = "0x58c - cgen_cfg3."]
    #[inline(always)]
    pub const fn cgen_cfg3(&self) -> &CGEN_CFG3 {
        &self.cgen_cfg3
    }
    #[doc = "0x600 - reg_sram_ret."]
    #[inline(always)]
    pub const fn reg_sram_ret(&self) -> &REG_SRAM_RET {
        &self.reg_sram_ret
    }
    #[doc = "0x604 - reg_sram_slp."]
    #[inline(always)]
    pub const fn reg_sram_slp(&self) -> &REG_SRAM_SLP {
        &self.reg_sram_slp
    }
    #[doc = "0x608 - reg_sram_parm."]
    #[inline(always)]
    pub const fn reg_sram_parm(&self) -> &REG_SRAM_PARM {
        &self.reg_sram_parm
    }
    #[doc = "0x60c - sram_cfg3."]
    #[inline(always)]
    pub const fn sram_cfg3(&self) -> &SRAM_CFG3 {
        &self.sram_cfg3
    }
    #[doc = "0x610 - reg_sram_parm2."]
    #[inline(always)]
    pub const fn reg_sram_parm2(&self) -> &REG_SRAM_PARM2 {
        &self.reg_sram_parm2
    }
    #[doc = "0x620 - Pseudo Static Random-Access Memory configuration"]
    #[inline(always)]
    pub const fn psram_config(&self) -> &PSRAM_CONFIG {
        &self.psram_config
    }
    #[doc = "0x6f0 - proc_mon."]
    #[inline(always)]
    pub const fn proc_mon(&self) -> &PROC_MON {
        &self.proc_mon
    }
    #[doc = "0x810 - Wireless Fidelity Phase-Locked Loop configuration 0"]
    #[inline(always)]
    pub const fn wifi_pll_config_0(&self) -> &WIFI_PLL_CONFIG_0 {
        &self.wifi_pll_config_0
    }
    #[doc = "0x814 - Wireless Fidelity Phase-Locked Loop configuration 1"]
    #[inline(always)]
    pub const fn wifi_pll_config_1(&self) -> &WIFI_PLL_CONFIG_1 {
        &self.wifi_pll_config_1
    }
    #[doc = "0x818 - Wireless Fidelity Phase-Locked Loop configuration 2"]
    #[inline(always)]
    pub const fn wifi_pll_config_2(&self) -> &WIFI_PLL_CONFIG_2 {
        &self.wifi_pll_config_2
    }
    #[doc = "0x81c - Wireless Fidelity Phase-Locked Loop configuration 3"]
    #[inline(always)]
    pub const fn wifi_pll_config_3(&self) -> &WIFI_PLL_CONFIG_3 {
        &self.wifi_pll_config_3
    }
    #[doc = "0x820 - Wireless Fidelity Phase-Locked Loop configuration 4"]
    #[inline(always)]
    pub const fn wifi_pll_config_4(&self) -> &WIFI_PLL_CONFIG_4 {
        &self.wifi_pll_config_4
    }
    #[doc = "0x824 - Wireless Fidelity Phase-Locked Loop configuration 5"]
    #[inline(always)]
    pub const fn wifi_pll_config_5(&self) -> &WIFI_PLL_CONFIG_5 {
        &self.wifi_pll_config_5
    }
    #[doc = "0x828 - Wireless Fidelity Phase-Locked Loop configuration 6"]
    #[inline(always)]
    pub const fn wifi_pll_config_6(&self) -> &WIFI_PLL_CONFIG_6 {
        &self.wifi_pll_config_6
    }
    #[doc = "0x82c - Wireless Fidelity Phase-Locked Loop configuration 7"]
    #[inline(always)]
    pub const fn wifi_pll_config_7(&self) -> &WIFI_PLL_CONFIG_7 {
        &self.wifi_pll_config_7
    }
    #[doc = "0x830 - Wireless Fidelity Phase-Locked Loop configuration 8"]
    #[inline(always)]
    pub const fn wifi_pll_config_8(&self) -> &WIFI_PLL_CONFIG_8 {
        &self.wifi_pll_config_8
    }
    #[doc = "0x834 - Wireless Fidelity Phase-Locked Loop configuration 9"]
    #[inline(always)]
    pub const fn wifi_pll_config_9(&self) -> &WIFI_PLL_CONFIG_9 {
        &self.wifi_pll_config_9
    }
    #[doc = "0x838 - Wireless Fidelity Phase-Locked Loop configuration 10"]
    #[inline(always)]
    pub const fn wifi_pll_config_10(&self) -> &WIFI_PLL_CONFIG_10 {
        &self.wifi_pll_config_10
    }
    #[doc = "0x83c - Wireless Fidelity Phase-Locked Loop configuration 11"]
    #[inline(always)]
    pub const fn wifi_pll_config_11(&self) -> &WIFI_PLL_CONFIG_11 {
        &self.wifi_pll_config_11
    }
    #[doc = "0x840 - Wireless Fidelity Phase-Locked Loop configuration 12"]
    #[inline(always)]
    pub const fn wifi_pll_config_12(&self) -> &WIFI_PLL_CONFIG_12 {
        &self.wifi_pll_config_12
    }
    #[doc = "0x844 - Wireless Fidelity Phase-Locked Loop configuration 13"]
    #[inline(always)]
    pub const fn wifi_pll_config_13(&self) -> &WIFI_PLL_CONFIG_13 {
        &self.wifi_pll_config_13
    }
    #[doc = "0x848 - Wireless Fidelity Phase-Locked Loop configuration 14"]
    #[inline(always)]
    pub const fn wifi_pll_config_14(&self) -> &WIFI_PLL_CONFIG_14 {
        &self.wifi_pll_config_14
    }
    #[doc = "0x884 - 1.8-V Low Dropout Linear Regulator configuration"]
    #[inline(always)]
    pub const fn ldo18(&self) -> &LDO18 {
        &self.ldo18
    }
    #[doc = "0x8c4..0x950 - Generic Purpose Input/Output config"]
    #[inline(always)]
    pub const fn gpio_config(&self, n: usize) -> &GPIO_CONFIG {
        &self.gpio_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c4..0x950 - Generic Purpose Input/Output config"]
    #[inline(always)]
    pub fn gpio_config_iter(&self) -> impl Iterator<Item = &GPIO_CONFIG> {
        self.gpio_config.iter()
    }
    #[doc = "0xac4 - Read value from Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)"]
    #[inline(always)]
    pub const fn gpio_input_0(&self) -> &GPIO_INPUT_0 {
        &self.gpio_input_0
    }
    #[doc = "0xac8 - Read value from Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)"]
    #[inline(always)]
    pub const fn gpio_input_1(&self) -> &GPIO_INPUT_1 {
        &self.gpio_input_1
    }
    #[doc = "0xae4 - Write value to Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)"]
    #[inline(always)]
    pub const fn gpio_output_0(&self) -> &GPIO_OUTPUT_0 {
        &self.gpio_output_0
    }
    #[doc = "0xae8 - Write value to Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)"]
    #[inline(always)]
    pub const fn gpio_output_1(&self) -> &GPIO_OUTPUT_1 {
        &self.gpio_output_1
    }
    #[doc = "0xaec - Set pin output value to high (GPIO0 ~ GPIO31)"]
    #[inline(always)]
    pub const fn gpio_set_0(&self) -> &GPIO_SET_0 {
        &self.gpio_set_0
    }
    #[doc = "0xaf0 - Set pin output value to high (GPIO32 ~ GPIO34)"]
    #[inline(always)]
    pub const fn gpio_set_1(&self) -> &GPIO_SET_1 {
        &self.gpio_set_1
    }
    #[doc = "0xaf4 - Set pin output value to low (GPIO0 ~ GPIO31)"]
    #[inline(always)]
    pub const fn gpio_clear_0(&self) -> &GPIO_CLEAR_0 {
        &self.gpio_clear_0
    }
    #[doc = "0xaf8 - Set pin output value to low (GPIO32 ~ GPIO34)"]
    #[inline(always)]
    pub const fn gpio_clear_1(&self) -> &GPIO_CLEAR_1 {
        &self.gpio_clear_1
    }
}
#[doc = "chip_inform (rw) register accessor: Chip information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chip_inform::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_inform::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_inform`]
module"]
pub type CHIP_INFORM = crate::Reg<chip_inform::CHIP_INFORM_SPEC>;
#[doc = "Chip information register"]
pub mod chip_inform;
#[doc = "interrupt_state (rw) register accessor: Chip interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_state`]
module"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Chip interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask (rw) register accessor: Chip interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Chip interrupt mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear (rw) register accessor: Chip clear interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clear`]
module"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Chip clear interrupt register"]
pub mod interrupt_clear;
#[doc = "clock_config_0 (rw) register accessor: System clock configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_config_0`]
module"]
pub type CLOCK_CONFIG_0 = crate::Reg<clock_config_0::CLOCK_CONFIG_0_SPEC>;
#[doc = "System clock configuration register 0"]
pub mod clock_config_0;
#[doc = "clock_config_1 (rw) register accessor: System clock configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_config_1`]
module"]
pub type CLOCK_CONFIG_1 = crate::Reg<clock_config_1::CLOCK_CONFIG_1_SPEC>;
#[doc = "System clock configuration register 1"]
pub mod clock_config_1;
#[doc = "bus_config_0 (rw) register accessor: Bus configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_config_0`]
module"]
pub type BUS_CONFIG_0 = crate::Reg<bus_config_0::BUS_CONFIG_0_SPEC>;
#[doc = "Bus configuration register 0"]
pub mod bus_config_0;
#[doc = "gpadc_config (rw) register accessor: General Purpose Analog-to-digital convert configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_config`]
module"]
pub type GPADC_CONFIG = crate::Reg<gpadc_config::GPADC_CONFIG_SPEC>;
#[doc = "General Purpose Analog-to-digital convert configuration"]
pub mod gpadc_config;
#[doc = "gpdac_config_0 (rw) register accessor: General Purpose Digital-to-analog convert configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config_0`]
module"]
pub type GPDAC_CONFIG_0 = crate::Reg<gpdac_config_0::GPDAC_CONFIG_0_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 0"]
pub mod gpdac_config_0;
#[doc = "gpdac_config_1 (rw) register accessor: General Purpose Digital-to-analog convert configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config_1`]
module"]
pub type GPDAC_CONFIG_1 = crate::Reg<gpdac_config_1::GPDAC_CONFIG_1_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 1"]
pub mod gpdac_config_1;
#[doc = "gpdac_config_2 (rw) register accessor: General Purpose Digital-to-analog convert configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config_2`]
module"]
pub type GPDAC_CONFIG_2 = crate::Reg<gpdac_config_2::GPDAC_CONFIG_2_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 2"]
pub mod gpdac_config_2;
#[doc = "gpdac_config_3 (rw) register accessor: General Purpose Digital-to-analog convert configuration 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config_3`]
module"]
pub type GPDAC_CONFIG_3 = crate::Reg<gpdac_config_3::GPDAC_CONFIG_3_SPEC>;
#[doc = "General Purpose Digital-to-analog convert configuration 3"]
pub mod gpdac_config_3;
#[doc = "dma_config_0 (rw) register accessor: Direct Memory Access configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_config_0`]
module"]
pub type DMA_CONFIG_0 = crate::Reg<dma_config_0::DMA_CONFIG_0_SPEC>;
#[doc = "Direct Memory Access configuration 0"]
pub mod dma_config_0;
#[doc = "dma_config_1 (rw) register accessor: Direct Memory Access configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_config_1`]
module"]
pub type DMA_CONFIG_1 = crate::Reg<dma_config_1::DMA_CONFIG_1_SPEC>;
#[doc = "Direct Memory Access configuration 1"]
pub mod dma_config_1;
#[doc = "dma_config_2 (rw) register accessor: Direct Memory Access configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_config_2`]
module"]
pub type DMA_CONFIG_2 = crate::Reg<dma_config_2::DMA_CONFIG_2_SPEC>;
#[doc = "Direct Memory Access configuration 2"]
pub mod dma_config_2;
#[doc = "ir_config_0 (rw) register accessor: Infrared configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_config_0`]
module"]
pub type IR_CONFIG_0 = crate::Reg<ir_config_0::IR_CONFIG_0_SPEC>;
#[doc = "Infrared configuration register 0"]
pub mod ir_config_0;
#[doc = "ir_config_1 (rw) register accessor: Infrared configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_config_1`]
module"]
pub type IR_CONFIG_1 = crate::Reg<ir_config_1::IR_CONFIG_1_SPEC>;
#[doc = "Infrared configuration register 1"]
pub mod ir_config_1;
#[doc = "uart_config (rw) register accessor: Universal Asynchronous Receiver/Transmitter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_config`]
module"]
pub type UART_CONFIG = crate::Reg<uart_config::UART_CONFIG_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter configuration"]
pub mod uart_config;
#[doc = "uart_signal_0 (rw) register accessor: Universal Asynchronous Receiver/Transmitter signal configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_signal_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_signal_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_signal_0`]
module"]
pub type UART_SIGNAL_0 = crate::Reg<uart_signal_0::UART_SIGNAL_0_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 0"]
pub mod uart_signal_0;
#[doc = "uart_signal_1 (rw) register accessor: Universal Asynchronous Receiver/Transmitter signal configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_signal_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_signal_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_signal_1`]
module"]
pub type UART_SIGNAL_1 = crate::Reg<uart_signal_1::UART_SIGNAL_1_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 1"]
pub mod uart_signal_1;
#[doc = "flash_config (rw) register accessor: Serial flash configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_config`]
module"]
pub type FLASH_CONFIG = crate::Reg<flash_config::FLASH_CONFIG_SPEC>;
#[doc = "Serial flash configuration"]
pub mod flash_config;
#[doc = "i2c_config (rw) register accessor: Inter-Integrated Circuit bus configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_config`]
module"]
pub type I2C_CONFIG = crate::Reg<i2c_config::I2C_CONFIG_SPEC>;
#[doc = "Inter-Integrated Circuit bus configuration"]
pub mod i2c_config;
#[doc = "i2s_config (rw) register accessor: Inter-IC Sound configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_config`]
module"]
pub type I2S_CONFIG = crate::Reg<i2s_config::I2S_CONFIG_SPEC>;
#[doc = "Inter-IC Sound configuration"]
pub mod i2s_config;
#[doc = "spi_config (rw) register accessor: Serial Peripheral Interface configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_config`]
module"]
pub type SPI_CONFIG = crate::Reg<spi_config::SPI_CONFIG_SPEC>;
#[doc = "Serial Peripheral Interface configuration"]
pub mod spi_config;
#[doc = "pwm_config (rw) register accessor: Pulse-Width configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_config`]
module"]
pub type PWM_CONFIG = crate::Reg<pwm_config::PWM_CONFIG_SPEC>;
#[doc = "Pulse-Width configuration"]
pub mod pwm_config;
#[doc = "dbi_config (rw) register accessor: MIPI Display Bus Interface clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_config`]
module"]
pub type DBI_CONFIG = crate::Reg<dbi_config::DBI_CONFIG_SPEC>;
#[doc = "MIPI Display Bus Interface clock configuration"]
pub mod dbi_config;
#[doc = "digit_clock_0 (rw) register accessor: Digital clock configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digit_clock_0`]
module"]
pub type DIGIT_CLOCK_0 = crate::Reg<digit_clock_0::DIGIT_CLOCK_0_SPEC>;
#[doc = "Digital clock configuration 0"]
pub mod digit_clock_0;
#[doc = "digit_clock_1 (rw) register accessor: Digital clock configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digit_clock_1`]
module"]
pub type DIGIT_CLOCK_1 = crate::Reg<digit_clock_1::DIGIT_CLOCK_1_SPEC>;
#[doc = "Digital clock configuration 1"]
pub mod digit_clock_1;
#[doc = "digit_clock_2 (rw) register accessor: Digital clock configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digit_clock_2`]
module"]
pub type DIGIT_CLOCK_2 = crate::Reg<digit_clock_2::DIGIT_CLOCK_2_SPEC>;
#[doc = "Digital clock configuration 2"]
pub mod digit_clock_2;
#[doc = "radio_config (rw) register accessor: Radio frequency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`radio_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`radio_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radio_config`]
module"]
pub type RADIO_CONFIG = crate::Reg<radio_config::RADIO_CONFIG_SPEC>;
#[doc = "Radio frequency configuration register"]
pub mod radio_config;
#[doc = "debug_config_0 (rw) register accessor: Debug configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_config_0`]
module"]
pub type DEBUG_CONFIG_0 = crate::Reg<debug_config_0::DEBUG_CONFIG_0_SPEC>;
#[doc = "Debug configuration register 0"]
pub mod debug_config_0;
#[doc = "debug_config_1 (rw) register accessor: Debug configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_config_1`]
module"]
pub type DEBUG_CONFIG_1 = crate::Reg<debug_config_1::DEBUG_CONFIG_1_SPEC>;
#[doc = "Debug configuration register 1"]
pub mod debug_config_1;
#[doc = "debug_config_2 (rw) register accessor: Debug configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_config_2`]
module"]
pub type DEBUG_CONFIG_2 = crate::Reg<debug_config_2::DEBUG_CONFIG_2_SPEC>;
#[doc = "Debug configuration register 2"]
pub mod debug_config_2;
#[doc = "debug_config_3 (rw) register accessor: Debug configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_config_3`]
module"]
pub type DEBUG_CONFIG_3 = crate::Reg<debug_config_3::DEBUG_CONFIG_3_SPEC>;
#[doc = "Debug configuration register 3"]
pub mod debug_config_3;
#[doc = "debug_config_4 (rw) register accessor: Debug configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_config_4`]
module"]
pub type DEBUG_CONFIG_4 = crate::Reg<debug_config_4::DEBUG_CONFIG_4_SPEC>;
#[doc = "Debug configuration register 4"]
pub mod debug_config_4;
#[doc = "self_test_0 (rw) register accessor: Machine Built-in Self Test register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`self_test_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`self_test_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@self_test_0`]
module"]
pub type SELF_TEST_0 = crate::Reg<self_test_0::SELF_TEST_0_SPEC>;
#[doc = "Machine Built-in Self Test register 0"]
pub mod self_test_0;
#[doc = "self_test_1 (rw) register accessor: Machine Built-in Self Test register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`self_test_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`self_test_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@self_test_1`]
module"]
pub type SELF_TEST_1 = crate::Reg<self_test_1::SELF_TEST_1_SPEC>;
#[doc = "Machine Built-in Self Test register 1"]
pub mod self_test_1;
#[doc = "audio_config_0 (rw) register accessor: Audio configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_config_0`]
module"]
pub type AUDIO_CONFIG_0 = crate::Reg<audio_config_0::AUDIO_CONFIG_0_SPEC>;
#[doc = "Audio configuration register 0"]
pub mod audio_config_0;
#[doc = "audio_config_1 (rw) register accessor: Audio configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_config_1`]
module"]
pub type AUDIO_CONFIG_1 = crate::Reg<audio_config_1::AUDIO_CONFIG_1_SPEC>;
#[doc = "Audio configuration register 1"]
pub mod audio_config_1;
#[doc = "emac_config (rw) register accessor: Ethernet Media Access Control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_config`]
module"]
pub type EMAC_CONFIG = crate::Reg<emac_config::EMAC_CONFIG_SPEC>;
#[doc = "Ethernet Media Access Control configuration"]
pub mod emac_config;
#[doc = "sdh_config (rw) register accessor: Secure Digital host configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdh_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdh_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdh_config`]
module"]
pub type SDH_CONFIG = crate::Reg<sdh_config::SDH_CONFIG_SPEC>;
#[doc = "Secure Digital host configuration"]
pub mod sdh_config;
#[doc = "permit_config (rw) register accessor: Permission control peripheral configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`permit_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`permit_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@permit_config`]
module"]
pub type PERMIT_CONFIG = crate::Reg<permit_config::PERMIT_CONFIG_SPEC>;
#[doc = "Permission control peripheral configuration"]
pub mod permit_config;
#[doc = "psram_config (rw) register accessor: Pseudo Static Random-Access Memory configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psram_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psram_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_config`]
module"]
pub type PSRAM_CONFIG = crate::Reg<psram_config::PSRAM_CONFIG_SPEC>;
#[doc = "Pseudo Static Random-Access Memory configuration"]
pub mod psram_config;
#[doc = "wifi_pll_config_0 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_0`]
module"]
pub type WIFI_PLL_CONFIG_0 = crate::Reg<wifi_pll_config_0::WIFI_PLL_CONFIG_0_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 0"]
pub mod wifi_pll_config_0;
#[doc = "wifi_pll_config_1 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_1`]
module"]
pub type WIFI_PLL_CONFIG_1 = crate::Reg<wifi_pll_config_1::WIFI_PLL_CONFIG_1_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 1"]
pub mod wifi_pll_config_1;
#[doc = "wifi_pll_config_2 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_2`]
module"]
pub type WIFI_PLL_CONFIG_2 = crate::Reg<wifi_pll_config_2::WIFI_PLL_CONFIG_2_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 2"]
pub mod wifi_pll_config_2;
#[doc = "wifi_pll_config_3 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_3`]
module"]
pub type WIFI_PLL_CONFIG_3 = crate::Reg<wifi_pll_config_3::WIFI_PLL_CONFIG_3_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 3"]
pub mod wifi_pll_config_3;
#[doc = "wifi_pll_config_4 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_4`]
module"]
pub type WIFI_PLL_CONFIG_4 = crate::Reg<wifi_pll_config_4::WIFI_PLL_CONFIG_4_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 4"]
pub mod wifi_pll_config_4;
#[doc = "wifi_pll_config_5 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_5`]
module"]
pub type WIFI_PLL_CONFIG_5 = crate::Reg<wifi_pll_config_5::WIFI_PLL_CONFIG_5_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 5"]
pub mod wifi_pll_config_5;
#[doc = "wifi_pll_config_6 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_6`]
module"]
pub type WIFI_PLL_CONFIG_6 = crate::Reg<wifi_pll_config_6::WIFI_PLL_CONFIG_6_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 6"]
pub mod wifi_pll_config_6;
#[doc = "wifi_pll_config_7 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_7`]
module"]
pub type WIFI_PLL_CONFIG_7 = crate::Reg<wifi_pll_config_7::WIFI_PLL_CONFIG_7_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 7"]
pub mod wifi_pll_config_7;
#[doc = "wifi_pll_config_8 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_8`]
module"]
pub type WIFI_PLL_CONFIG_8 = crate::Reg<wifi_pll_config_8::WIFI_PLL_CONFIG_8_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 8"]
pub mod wifi_pll_config_8;
#[doc = "wifi_pll_config_9 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_9`]
module"]
pub type WIFI_PLL_CONFIG_9 = crate::Reg<wifi_pll_config_9::WIFI_PLL_CONFIG_9_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 9"]
pub mod wifi_pll_config_9;
#[doc = "wifi_pll_config_10 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_10`]
module"]
pub type WIFI_PLL_CONFIG_10 = crate::Reg<wifi_pll_config_10::WIFI_PLL_CONFIG_10_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 10"]
pub mod wifi_pll_config_10;
#[doc = "wifi_pll_config_11 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_11`]
module"]
pub type WIFI_PLL_CONFIG_11 = crate::Reg<wifi_pll_config_11::WIFI_PLL_CONFIG_11_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 11"]
pub mod wifi_pll_config_11;
#[doc = "wifi_pll_config_12 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_12`]
module"]
pub type WIFI_PLL_CONFIG_12 = crate::Reg<wifi_pll_config_12::WIFI_PLL_CONFIG_12_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 12"]
pub mod wifi_pll_config_12;
#[doc = "wifi_pll_config_13 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_13`]
module"]
pub type WIFI_PLL_CONFIG_13 = crate::Reg<wifi_pll_config_13::WIFI_PLL_CONFIG_13_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 13"]
pub mod wifi_pll_config_13;
#[doc = "wifi_pll_config_14 (rw) register accessor: Wireless Fidelity Phase-Locked Loop configuration 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_config_14`]
module"]
pub type WIFI_PLL_CONFIG_14 = crate::Reg<wifi_pll_config_14::WIFI_PLL_CONFIG_14_SPEC>;
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 14"]
pub mod wifi_pll_config_14;
#[doc = "ldo18 (rw) register accessor: 1.8-V Low Dropout Linear Regulator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo18`]
module"]
pub type LDO18 = crate::Reg<ldo18::LDO18_SPEC>;
#[doc = "1.8-V Low Dropout Linear Regulator configuration"]
pub mod ldo18;
#[doc = "gpio_config (rw) register accessor: Generic Purpose Input/Output config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_config`]
module"]
pub type GPIO_CONFIG = crate::Reg<gpio_config::GPIO_CONFIG_SPEC>;
#[doc = "Generic Purpose Input/Output config"]
pub mod gpio_config;
#[doc = "core_cfg16 (rw) register accessor: core_cfg16.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg16`]
module"]
pub type CORE_CFG16 = crate::Reg<core_cfg16::CORE_CFG16_SPEC>;
#[doc = "core_cfg16."]
pub mod core_cfg16;
#[doc = "core_cfg17 (rw) register accessor: core_cfg17.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg17`]
module"]
pub type CORE_CFG17 = crate::Reg<core_cfg17::CORE_CFG17_SPEC>;
#[doc = "core_cfg17."]
pub mod core_cfg17;
#[doc = "core_cfg18 (rw) register accessor: core_cfg18.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg18`]
module"]
pub type CORE_CFG18 = crate::Reg<core_cfg18::CORE_CFG18_SPEC>;
#[doc = "core_cfg18."]
pub mod core_cfg18;
#[doc = "core_cfg19 (rw) register accessor: core_cfg19.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg19`]
module"]
pub type CORE_CFG19 = crate::Reg<core_cfg19::CORE_CFG19_SPEC>;
#[doc = "core_cfg19."]
pub mod core_cfg19;
#[doc = "core_cfg20 (rw) register accessor: core_cfg20.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg20`]
module"]
pub type CORE_CFG20 = crate::Reg<core_cfg20::CORE_CFG20_SPEC>;
#[doc = "core_cfg20."]
pub mod core_cfg20;
#[doc = "core_cfg21 (rw) register accessor: core_cfg21.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_cfg21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_cfg21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg21`]
module"]
pub type CORE_CFG21 = crate::Reg<core_cfg21::CORE_CFG21_SPEC>;
#[doc = "core_cfg21."]
pub mod core_cfg21;
#[doc = "pio_cfg0 (rw) register accessor: pio_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio_cfg0`]
module"]
pub type PIO_CFG0 = crate::Reg<pio_cfg0::PIO_CFG0_SPEC>;
#[doc = "pio_cfg0."]
pub mod pio_cfg0;
#[doc = "pwm_cfg0 (rw) register accessor: pwm_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_cfg0`]
module"]
pub type PWM_CFG0 = crate::Reg<pwm_cfg0::PWM_CFG0_SPEC>;
#[doc = "pwm_cfg0."]
pub mod pwm_cfg0;
#[doc = "bmx_cfg0 (rw) register accessor: bmx_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg0`]
module"]
pub type BMX_CFG0 = crate::Reg<bmx_cfg0::BMX_CFG0_SPEC>;
#[doc = "bmx_cfg0."]
pub mod bmx_cfg0;
#[doc = "bmx_cfg1 (rw) register accessor: bmx_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg1`]
module"]
pub type BMX_CFG1 = crate::Reg<bmx_cfg1::BMX_CFG1_SPEC>;
#[doc = "bmx_cfg1."]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2 (rw) register accessor: bmx_cfg2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg2`]
module"]
pub type BMX_CFG2 = crate::Reg<bmx_cfg2::BMX_CFG2_SPEC>;
#[doc = "bmx_cfg2."]
pub mod bmx_cfg2;
#[doc = "bmx_cfg3 (rw) register accessor: bmx_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg3`]
module"]
pub type BMX_CFG3 = crate::Reg<bmx_cfg3::BMX_CFG3_SPEC>;
#[doc = "bmx_cfg3."]
pub mod bmx_cfg3;
#[doc = "bmx_cfg4 (rw) register accessor: bmx_cfg4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg4`]
module"]
pub type BMX_CFG4 = crate::Reg<bmx_cfg4::BMX_CFG4_SPEC>;
#[doc = "bmx_cfg4."]
pub mod bmx_cfg4;
#[doc = "bmx_cfg5 (rw) register accessor: bmx_cfg5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg5`]
module"]
pub type BMX_CFG5 = crate::Reg<bmx_cfg5::BMX_CFG5_SPEC>;
#[doc = "bmx_cfg5."]
pub mod bmx_cfg5;
#[doc = "bmx_cfg6 (rw) register accessor: bmx_cfg6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg6`]
module"]
pub type BMX_CFG6 = crate::Reg<bmx_cfg6::BMX_CFG6_SPEC>;
#[doc = "bmx_cfg6."]
pub mod bmx_cfg6;
#[doc = "cam_cfg0 (rw) register accessor: cam_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_cfg0`]
module"]
pub type CAM_CFG0 = crate::Reg<cam_cfg0::CAM_CFG0_SPEC>;
#[doc = "cam_cfg0."]
pub mod cam_cfg0;
#[doc = "sdio_cfg0 (rw) register accessor: sdio_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_cfg0`]
module"]
pub type SDIO_CFG0 = crate::Reg<sdio_cfg0::SDIO_CFG0_SPEC>;
#[doc = "sdio_cfg0."]
pub mod sdio_cfg0;
#[doc = "glb_parm_cfg0 (rw) register accessor: glb_parm_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_parm_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_parm_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_parm_cfg0`]
module"]
pub type GLB_PARM_CFG0 = crate::Reg<glb_parm_cfg0::GLB_PARM_CFG0_SPEC>;
#[doc = "glb_parm_cfg0."]
pub mod glb_parm_cfg0;
#[doc = "debug_cfg1 (rw) register accessor: debug_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_cfg1`]
module"]
pub type DEBUG_CFG1 = crate::Reg<debug_cfg1::DEBUG_CFG1_SPEC>;
#[doc = "debug_cfg1."]
pub mod debug_cfg1;
#[doc = "reset_sts0 (rw) register accessor: reset_sts0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_sts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_sts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_sts0`]
module"]
pub type RESET_STS0 = crate::Reg<reset_sts0::RESET_STS0_SPEC>;
#[doc = "reset_sts0."]
pub mod reset_sts0;
#[doc = "swrst_cfg0 (rw) register accessor: swrst_s1_ext + swrst_s3 + swrst_s2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg0`]
module"]
pub type SWRST_CFG0 = crate::Reg<swrst_cfg0::SWRST_CFG0_SPEC>;
#[doc = "swrst_s1_ext + swrst_s3 + swrst_s2"]
pub mod swrst_cfg0;
#[doc = "swrst_s1 (rw) register accessor: swrst_s1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_s1`]
module"]
pub type SWRST_S1 = crate::Reg<swrst_s1::SWRST_S1_SPEC>;
#[doc = "swrst_s1."]
pub mod swrst_s1;
#[doc = "swrst_cfg2 (rw) register accessor: swrst_cfg2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg2`]
module"]
pub type SWRST_CFG2 = crate::Reg<swrst_cfg2::SWRST_CFG2_SPEC>;
#[doc = "swrst_cfg2."]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3 (rw) register accessor: Disable hreset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg3`]
module"]
pub type SWRST_CFG3 = crate::Reg<swrst_cfg3::SWRST_CFG3_SPEC>;
#[doc = "Disable hreset"]
pub mod swrst_cfg3;
#[doc = "cgen_m (rw) register accessor: cgen_m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_m`]
module"]
pub type CGEN_M = crate::Reg<cgen_m::CGEN_M_SPEC>;
#[doc = "cgen_m."]
pub mod cgen_m;
#[doc = "cgen_cfg1 (rw) register accessor: cgen_s1a + cgen_s1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg1`]
module"]
pub type CGEN_CFG1 = crate::Reg<cgen_cfg1::CGEN_CFG1_SPEC>;
#[doc = "cgen_s1a + cgen_s1"]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2 (rw) register accessor: cgen_s1_ext + cgen_s3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg2`]
module"]
pub type CGEN_CFG2 = crate::Reg<cgen_cfg2::CGEN_CFG2_SPEC>;
#[doc = "cgen_s1_ext + cgen_s3"]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3 (rw) register accessor: cgen_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg3`]
module"]
pub type CGEN_CFG3 = crate::Reg<cgen_cfg3::CGEN_CFG3_SPEC>;
#[doc = "cgen_cfg3."]
pub mod cgen_cfg3;
#[doc = "reg_sram_ret (rw) register accessor: reg_sram_ret.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_ret::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_ret::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_sram_ret`]
module"]
pub type REG_SRAM_RET = crate::Reg<reg_sram_ret::REG_SRAM_RET_SPEC>;
#[doc = "reg_sram_ret."]
pub mod reg_sram_ret;
#[doc = "reg_sram_slp (rw) register accessor: reg_sram_slp.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_slp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_slp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_sram_slp`]
module"]
pub type REG_SRAM_SLP = crate::Reg<reg_sram_slp::REG_SRAM_SLP_SPEC>;
#[doc = "reg_sram_slp."]
pub mod reg_sram_slp;
#[doc = "reg_sram_parm (rw) register accessor: reg_sram_parm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_parm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_parm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_sram_parm`]
module"]
pub type REG_SRAM_PARM = crate::Reg<reg_sram_parm::REG_SRAM_PARM_SPEC>;
#[doc = "reg_sram_parm."]
pub mod reg_sram_parm;
#[doc = "sram_cfg3 (rw) register accessor: sram_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg3`]
module"]
pub type SRAM_CFG3 = crate::Reg<sram_cfg3::SRAM_CFG3_SPEC>;
#[doc = "sram_cfg3."]
pub mod sram_cfg3;
#[doc = "reg_sram_parm2 (rw) register accessor: reg_sram_parm2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_parm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_parm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_sram_parm2`]
module"]
pub type REG_SRAM_PARM2 = crate::Reg<reg_sram_parm2::REG_SRAM_PARM2_SPEC>;
#[doc = "reg_sram_parm2."]
pub mod reg_sram_parm2;
#[doc = "proc_mon (rw) register accessor: proc_mon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`proc_mon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_mon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@proc_mon`]
module"]
pub type PROC_MON = crate::Reg<proc_mon::PROC_MON_SPEC>;
#[doc = "proc_mon."]
pub mod proc_mon;
#[doc = "gpio_input_0 (rw) register accessor: Read value from Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_input_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_input_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_input_0`]
module"]
pub type GPIO_INPUT_0 = crate::Reg<gpio_input_0::GPIO_INPUT_0_SPEC>;
#[doc = "Read value from Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)"]
pub mod gpio_input_0;
#[doc = "gpio_input_1 (rw) register accessor: Read value from Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_input_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_input_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_input_1`]
module"]
pub type GPIO_INPUT_1 = crate::Reg<gpio_input_1::GPIO_INPUT_1_SPEC>;
#[doc = "Read value from Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)"]
pub mod gpio_input_1;
#[doc = "gpio_output_0 (rw) register accessor: Write value to Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_output_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_output_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_output_0`]
module"]
pub type GPIO_OUTPUT_0 = crate::Reg<gpio_output_0::GPIO_OUTPUT_0_SPEC>;
#[doc = "Write value to Generic Purpose Input/Output pins (GPIO0 ~ GPIO31)"]
pub mod gpio_output_0;
#[doc = "gpio_output_1 (rw) register accessor: Write value to Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_output_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_output_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_output_1`]
module"]
pub type GPIO_OUTPUT_1 = crate::Reg<gpio_output_1::GPIO_OUTPUT_1_SPEC>;
#[doc = "Write value to Generic Purpose Input/Output pins (GPIO32 ~ GPIO34)"]
pub mod gpio_output_1;
#[doc = "gpio_set_0 (rw) register accessor: Set pin output value to high (GPIO0 ~ GPIO31)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_set_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_set_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_set_0`]
module"]
pub type GPIO_SET_0 = crate::Reg<gpio_set_0::GPIO_SET_0_SPEC>;
#[doc = "Set pin output value to high (GPIO0 ~ GPIO31)"]
pub mod gpio_set_0;
#[doc = "gpio_set_1 (rw) register accessor: Set pin output value to high (GPIO32 ~ GPIO34)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_set_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_set_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_set_1`]
module"]
pub type GPIO_SET_1 = crate::Reg<gpio_set_1::GPIO_SET_1_SPEC>;
#[doc = "Set pin output value to high (GPIO32 ~ GPIO34)"]
pub mod gpio_set_1;
#[doc = "gpio_clear_0 (rw) register accessor: Set pin output value to low (GPIO0 ~ GPIO31)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_clear_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_clear_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_clear_0`]
module"]
pub type GPIO_CLEAR_0 = crate::Reg<gpio_clear_0::GPIO_CLEAR_0_SPEC>;
#[doc = "Set pin output value to low (GPIO0 ~ GPIO31)"]
pub mod gpio_clear_0;
#[doc = "gpio_clear_1 (rw) register accessor: Set pin output value to low (GPIO32 ~ GPIO34)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_clear_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_clear_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_clear_1`]
module"]
pub type GPIO_CLEAR_1 = crate::Reg<gpio_clear_1::GPIO_CLEAR_1_SPEC>;
#[doc = "Set pin output value to low (GPIO32 ~ GPIO34)"]
pub mod gpio_clear_1;
