pub mod baud_rate_divisor_config;
pub mod clk_config;
pub mod fifo_config;
pub mod oversampling_config;
pub mod uart_config;

#[repr(C)]
struct UartGprcmRegs {
    pwren: u32,  // 0x00000800 Power enable
    rstctl: u32, // 0x00000804 Reset Control
    clkcfg: u32, // 0x00000808 Peripheral Clock Configuration Register
    reserved0: [u32; 2],
    stat: u32, // 0x00000814 Status Register
}

#[repr(C)]
struct UartCpuIntRegs {
    iidx: u32, // 0x00001020 Interrupt index
    reserved0: u32,
    imask: u32, // 0x00001028 Interrupt mask
    reserved1: u32,
    ris: u32, // 0x00001030 Raw interrupt status
    reserved2: u32,
    mis: u32, // 0x00001038 Masked interrupt status
    reserved3: u32,
    iset: u32, // 0x00001040 Interrupt set
    reserved4: u32,
    iclr: u32, // 0x00001048 Interrupt clear
}

#[repr(C)]
struct UartDmaTrigTxRegs {
    iidx: u32, // 0x00001080 Interrupt index
    reserved0: u32,
    imask: u32, // 0x00001088 Interrupt mask
    reserved1: u32,
    ris: u32, // 0x00001090 Raw interrupt status
    reserved2: u32,
    mis: u32, // 0x00001098 Masked interrupt status
    reserved3: u32,
    iset: u32, // 0x000010A0 Interrupt set
    reserved4: u32,
    iclr: u32, // 0x000010A8 Interrupt clear
}

#[repr(C)]
struct UartDmaTrigRxRegs {
    iidx: u32, // 0x00001050 Interrupt index
    reserved0: u32,
    imask: u32, // 0x00001058 Interrupt mask
    reserved1: u32,
    ris: u32, // 0x00001060 Raw interrupt status
    reserved2: u32,
    mis: u32, // 0x00001068 Masked interrupt status
    reserved3: u32,
    iset: u32, // 0x00001070 Interrupt set
    reserved4: u32,
    iclr: u32, // 0x00001078 Interrupt clear
}

#[repr(C)]
pub struct UartRegs {
    reserved0: [u32; 512],
    gprcm: UartGprcmRegs, // 0x00000800
    reserved1: [u32; 506],
    clkdiv: u32, // 0x00001000 Clock Divider
    reserved2: u32,
    clksel: u32, // 0x00001008 Clock Select for ULP peripherals
    reserved3: [u32; 3],
    pdbgctl: u32, // 0x00001018 Peripheral Debug Control
    reserved4: u32,
    cpu_int: UartCpuIntRegs, // 0x00001020
    reserved5: u32,
    dma_trig_rx: UartDmaTrigRxRegs, // 0x00001050
    reserved6: u32,
    dma_trig_tx: UartDmaTrigTxRegs, // 0x00001080
    reserved7: [u32; 13],
    evt_mode: u32, // 0x000010E0 Event Mode
    intctl: u32,   // 0x000010E4 Interrupt Control Register
    reserved8: [u32; 6],
    ctl0: u32,  // 0x00001100 UART Control Register 0
    lcrh: u32,  // 0x00001104 UART Line Control Register
    stat: u32,  // 0x00001108 UART Status Register
    ifls: u32,  // 0x0000110C FIFO Level Select Register
    ibrd: u32,  // 0x00001110 Integer Baud Rate Divisor
    fbrd: u32,  // 0x00001114 Fractional Baud Rate Divisor
    gfctl: u32, // 0x00001118 Glitch Filter Control
    reserved9: u32,
    txdata: u32, // 0x00001120 Transmit Data Register
    rxdata: u32, // 0x00001124 Receive Data Register
    reserved10: [u32; 2],
    lincnt: u32, // 0x00001130 LIN Mode Counter Register
    linctl: u32, // 0x00001134 LIN Mode Control Register
    linc0: u32,  // 0x00001138 LIN Mode Capture 0
    linc1: u32,  // 0x0000113C LIN Mode Capture 1
    irctl: u32,  // 0x00001140 IrDA Control Word
    reserved11: u32,
    amask: u32, // 0x00001148 Self Address Mask Register
    addr: u32,  // 0x0000114C Self Address Register
    reserved12: [u32; 4],
    clkdiv2: u32, // 0x00001160 Clock Divider 2
}

impl UartRegs {
    pub fn reset(&mut self) {
        const RSTCTL_KEY_UNLOCK: u32 = 0xB100_0000;
        const RSTCTL_STKYCLR: u32 = 0x0000_0002;
        const RSTCTL_ASSERT: u32 = 0x0000_0001;

        self.gprcm.rstctl = RSTCTL_KEY_UNLOCK | RSTCTL_STKYCLR | RSTCTL_ASSERT;
    }

    pub fn enable_power(&mut self) {
        const PWREN_KEY_UNLOCK: u32 = 0x2600_0000;
        const PWREN_ENABLE: u32 = 0x0000_0001;

        self.gprcm.pwren = PWREN_KEY_UNLOCK | PWREN_ENABLE;
    }

    pub fn enable(&mut self) {
        const CTL0_ENABLE: u32 = 0x0000_0001;
        self.ctl0 |= CTL0_ENABLE;
    }

    pub fn disable(&mut self) {
        const CTL0_ENABLE_MASK: u32 = 0x0000_0001;
        self.ctl0 &= !CTL0_ENABLE_MASK;
    }

    #[inline(always)]
    pub fn transmit(&mut self, data: u8) {
        self.txdata = data as u32;
    }

    pub fn transmit_str(&mut self, data: &str) {
        let num_chunks = data.len() / 4 - (data.len()%4 == 0) as usize;
        for i in 0..=num_chunks {
            let c1: u32 = data.bytes().nth(i * 4 + 0).unwrap_or(0x20).into();
            let c2: u32 = data.bytes().nth(i * 4 + 1).unwrap_or(0x20).into();
            let c3: u32 = data.bytes().nth(i * 4 + 2).unwrap_or(0x20).into();
            let c4: u32 = data.bytes().nth(i * 4 + 3).unwrap_or(0x20).into();

            // self.txdata = c1 << 24 | c1 << 16 | c1 << 8 | c1;
            // self.txdata = c2 << 24 | c2 << 16 | c2 << 8 | c2;
            // self.txdata = c3 << 24 | c3 << 16 | c3 << 8 | c3;
            // self.txdata = c4 << 24 | c4 << 16 | c4 << 8 | c4;
            //
            //

            self.txdata = c1;
            self.txdata = c2;
            self.txdata = c3;
            self.txdata = c4;

            for _ in 0..(33 * 20) {
                unsafe { core::arch::asm!("nop") };
            }
        }
    }

    pub fn recieve_byte_blocking(&self) -> char {
        while self.is_rxfifo_empty() {}

        self.recieve_data()
    }

    fn recieve_data(&self) -> char {
        const UART_RXDATA_DATA_MASK: u32 = 0x000000FF;
        (self.rxdata & UART_RXDATA_DATA_MASK) as u8 as char
    }

    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        const UART_STAT_TXFF_MASK: u32 = 0x00000080;
        const UART_STAT_TXFF_SET: u32 = 0x00000080;

        (self.stat & UART_STAT_TXFF_MASK) == UART_STAT_TXFF_SET
    }

    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        const UART_STAT_RXFE_MASK: u32 = 0x00000004;
        const UART_STAT_RXFE_SET: u32 = 0x00000004;

        (self.stat & UART_STAT_RXFE_MASK) == UART_STAT_RXFE_SET
    }
}

use crate::utils::MemoryMapped;
impl MemoryMapped for UartRegs {}

// pub mod baud_rate_divisor_config;
// pub mod clk_config;
// pub mod fifo_config;
// pub mod oversampling_config;
// pub mod uart_config;
//
// #[repr(C)]
// struct UART_GPRCM_Regs {
// PWREN: u32,  /* !< (@ 0x00000800) Power enable */
// RSTCTL: u32, /* !< (@ 0x00000804) Reset Control */
// CLKCFG: u32, /* !< (@ 0x00000808) Peripheral Clock Configuration Register */
// reserved0: [u32; 2],
// STAT: u32, /* !< (@ 0x00000814) Status Register */
// }
//
// #[repr(C)]
// struct UART_CPU_INT_Regs {
// IIDX: u32, /* !< (@ 0x00001020) Interrupt index */
// RESERVED0: u32,
// IMASK: u32, /* !< (@ 0x00001028) Interrupt mask */
// RESERVED1: u32,
// RIS: u32, /* !< (@ 0x00001030) Raw interrupt status */
// RESERVED2: u32,
// MIS: u32, /* !< (@ 0x00001038) Masked interrupt status */
// RESERVED3: u32,
// ISET: u32, /* !< (@ 0x00001040) Interrupt set */
// RESERVED4: u32,
// ICLR: u32, /* !< (@ 0x00001048) Interrupt clear */
// }
//
// #[repr(C)]
// struct UART_DMA_TRIG_TX_Regs {
// IIDX: u32, /* !< (@ 0x00001080) Interrupt index */
// RESERVED0: u32,
// IMASK: u32, /* !< (@ 0x00001088) Interrupt mask */
// RESERVED1: u32,
// RIS: u32, /* !< (@ 0x00001090) Raw interrupt status */
// RESERVED2: u32,
// MIS: u32, /* !< (@ 0x00001098) Masked interrupt status */
// RESERVED3: u32,
// ISET: u32, /* !< (@ 0x000010A0) Interrupt set */
// RESERVED4: u32,
// ICLR: u32, /* !< (@ 0x000010A8) Interrupt clear */
// }
//
// #[repr(C)]
// struct UART_DMA_TRIG_RX_Regs {
// IIDX: u32, /* !< (@ 0x00001050) Interrupt index */
// RESERVED0: u32,
// IMASK: u32, /* !< (@ 0x00001058) Interrupt mask */
// RESERVED1: u32,
// RIS: u32, /* !< (@ 0x00001060) Raw interrupt status */
// RESERVED2: u32,
// MIS: u32, /* !< (@ 0x00001068) Masked interrupt status */
// RESERVED3: u32,
// ISET: u32, /* !< (@ 0x00001070) Interrupt set */
// RESERVED4: u32,
// ICLR: u32, /* !< (@ 0x00001078) Interrupt clear */
// }
//
// #[repr(C)]
// pub struct UART_Regs {
// reserved0: [u32; 512],
// GPRCM: UART_GPRCM_Regs, /* !< (@ 0x00000800) */
// RESERVED1: [u32; 506],
// CLKDIV: u32, /* !< (@ 0x00001000) Clock Divider */
// RESERVED2: u32,
// CLKSEL: u32, /* !< (@ 0x00001008) Clock Select for Ultra Low Power peripherals */
// RESERVED3: [u32; 3],
// PDBGCTL: u32, /* !< (@ 0x00001018) Peripheral Debug Control */
// RESERVED4: u32,
// CPU_INT: UART_CPU_INT_Regs, /* !< (@ 0x00001020) */
// RESERVED5: u32,
// DMA_TRIG_RX: UART_DMA_TRIG_RX_Regs, /* !< (@ 0x00001050) */
// RESERVED6: u32,
// DMA_TRIG_TX: UART_DMA_TRIG_TX_Regs, /* !< (@ 0x00001080) */
// RESERVED7: [u32; 13],
// EVT_MODE: u32, /* !< (@ 0x000010E0) Event Mode */
// INTCTL: u32,   /* !< (@ 0x000010E4) Interrupt control register */
// RESERVED8: [u32; 6],
// CTL0: u32,  /* !< (@ 0x00001100) UART Control Register 0 */
// LCRH: u32,  /* !< (@ 0x00001104) UART Line Control Register */
// STAT: u32,  /* !< (@ 0x00001108) UART Status Register */
// IFLS: u32,  /* !< (@ 0x0000110C) UART Interrupt FIFO Level Select Register */
// IBRD: u32,  /* !< (@ 0x00001110) UART Integer Baud-Rate Divisor Register */
// FBRD: u32,  /* !< (@ 0x00001114) UART Fractional Baud-Rate Divisor Register */
// GFCTL: u32, /* !< (@ 0x00001118) Glitch Filter Control */
// RESERVED9: u32,
// TXDATA: u32, /* !< (@ 0x00001120) UART Transmit Data Register */
// RXDATA: u32, /* !< (@ 0x00001124) UART Receive Data Register */
// RESERVED10: [u32; 2],
// LINCNT: u32, /* !< (@ 0x00001130) UART LIN Mode Counter Register */
// LINCTL: u32, /* !< (@ 0x00001134) UART LIN Mode Control Register */
// LINC0: u32,  /* !< (@ 0x00001138) UART LIN Mode Capture 0 Register */
// LINC1: u32,  /* !< (@ 0x0000113C) UART LIN Mode Capture 1 Register */
// IRCTL: u32,  /* !< (@ 0x00001140) eUSCI_Ax IrDA Control Word Register */
// RESERVED11: u32,
// AMASK: u32, /* !< (@ 0x00001148) Self Address Mask Register */
// ADDR: u32,  /* !< (@ 0x0000114C) Self Address Register */
// RESERVED12: [u32; 4],
// CLKDIV2: u32, /* !< (@ 0x00001160) Clock Divider */
// }
//
// impl UART_Regs {
// pub fn reset(&mut self) {
// const UART_RSTCTL_KEY_UNLOCK_W: u32 = 0xB1000000;
// const UART_RSTCTL_RESETSTKYCLR_CLR: u32 = 0x00000002;
// const UART_RSTCTL_RESETASSERT_ASSERT: u32 = 0x00000001;
// self.GPRCM.RSTCTL = UART_RSTCTL_KEY_UNLOCK_W
// | UART_RSTCTL_RESETSTKYCLR_CLR
// | UART_RSTCTL_RESETASSERT_ASSERT;
// }
//
// pub fn enable_power(&mut self) {
// const UART_PWREN_KEY_UNLOCK_W: u32 = 0x26000000;
// const UART_PWREN_ENABLE_ENABLE: u32 = 0x00000001;
// self.GPRCM.PWREN = UART_PWREN_KEY_UNLOCK_W | UART_PWREN_ENABLE_ENABLE;
// }
//
// pub fn enable(&mut self) {
// const UART_CTL0_ENABLE_ENABLE: u32 = 0x00000001;
// self.CTL0 |= UART_CTL0_ENABLE_ENABLE;
// }
//
// pub fn disable(&mut self) {
// const UART_CTL0_ENABLE_MASK: u32 = 0x00000001;
// self.CTL0 &= !UART_CTL0_ENABLE_MASK;
// }
//
// #[inline(always)]
// pub fn transmit(&mut self, data: u8) {
// self.TXDATA = data as u32;
// }
//
// pub fn transmit_str(&mut self, data: &str) {
// for i in 0..=(data.len() / 4) {
// let c1: u32 = data.bytes().nth(i * 4 + 0).unwrap_or(0x20).into();
// let c2: u32 = data.bytes().nth(i * 4 + 1).unwrap_or(0x20).into();
// let c3: u32 = data.bytes().nth(i * 4 + 2).unwrap_or(0x20).into();
// let c4: u32 = data.bytes().nth(i * 4 + 3).unwrap_or(0x20).into();
//
// self.TXDATA = c1 << 24 | c1 << 16 | c1 << 8 | c1;
// self.TXDATA = c2 << 24 | c2 << 16 | c2 << 8 | c2;
// self.TXDATA = c3 << 24 | c3 << 16 | c3 << 8 | c3;
// self.TXDATA = c4 << 24 | c4 << 16 | c4 << 8 | c4;
//
// for _ in 0..(33 * 20) {
// unsafe { core::arch::asm!("nop") };
// }
// }
// }
// }
//
// use crate::utils::MemoryMapped;
// impl MemoryMapped for UART_Regs {}
