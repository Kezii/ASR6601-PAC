#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier: Ier,
    irer: Irer,
    iter: Iter,
    cer: Cer,
    ccr: Ccr,
    rxffr: Rxffr,
    txffr: Txffr,
    _reserved7: [u8; 0x04],
    lrbr_lthr: LrbrLthr,
    rrbr_rthr: RrbrRthr,
    rer: Rer,
    ter: Ter,
    rcr: Rcr,
    tcr: Tcr,
    isr: Isr,
    imr: Imr,
    ror: Ror,
    tor: Tor,
    rfcr: Rfcr,
    tfcr: Tfcr,
    rff: Rff,
    tff: Tff,
    _reserved21: [u8; 0x0168],
    rxdma: Rxdma,
    rrxdma: Rrxdma,
    txdma: Txdma,
    rtxdma: Rtxdma,
    _reserved25: [u8; 0x20],
    i2s_comp_param_2: I2sCompParam2,
    i2s_comp_param_1: I2sCompParam1,
    i2s_comp_version: I2sCompVersion,
    i2s_comp_type: I2sCompType,
}
impl RegisterBlock {
    #[doc = "0x00 - enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x04 - receiver block enable register"]
    #[inline(always)]
    pub const fn irer(&self) -> &Irer {
        &self.irer
    }
    #[doc = "0x08 - transmitter block enable register"]
    #[inline(always)]
    pub const fn iter(&self) -> &Iter {
        &self.iter
    }
    #[doc = "0x0c - clock enable register"]
    #[inline(always)]
    pub const fn cer(&self) -> &Cer {
        &self.cer
    }
    #[doc = "0x10 - clock configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x14 - receiver block FIFO reset register"]
    #[inline(always)]
    pub const fn rxffr(&self) -> &Rxffr {
        &self.rxffr
    }
    #[doc = "0x18 - transmitter block FIFO reset register"]
    #[inline(always)]
    pub const fn txffr(&self) -> &Txffr {
        &self.txffr
    }
    #[doc = "0x20 - right receive buffer register"]
    #[inline(always)]
    pub const fn lrbr_lthr(&self) -> &LrbrLthr {
        &self.lrbr_lthr
    }
    #[doc = "0x24 - right transmit holding register"]
    #[inline(always)]
    pub const fn rrbr_rthr(&self) -> &RrbrRthr {
        &self.rrbr_rthr
    }
    #[doc = "0x28 - receiver enable register"]
    #[inline(always)]
    pub const fn rer(&self) -> &Rer {
        &self.rer
    }
    #[doc = "0x2c - transmitter enable register"]
    #[inline(always)]
    pub const fn ter(&self) -> &Ter {
        &self.ter
    }
    #[doc = "0x30 - receiver configuration register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x34 - transmitter configuration register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x38 - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x3c - interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x40 - receiver overrun register"]
    #[inline(always)]
    pub const fn ror(&self) -> &Ror {
        &self.ror
    }
    #[doc = "0x44 - transmitter overrun register"]
    #[inline(always)]
    pub const fn tor(&self) -> &Tor {
        &self.tor
    }
    #[doc = "0x48 - receiver FIFO configuration register"]
    #[inline(always)]
    pub const fn rfcr(&self) -> &Rfcr {
        &self.rfcr
    }
    #[doc = "0x4c - transmitter FIFO configuration register"]
    #[inline(always)]
    pub const fn tfcr(&self) -> &Tfcr {
        &self.tfcr
    }
    #[doc = "0x50 - receiver FIFO flush register"]
    #[inline(always)]
    pub const fn rff(&self) -> &Rff {
        &self.rff
    }
    #[doc = "0x54 - transmitter FIFO flush register"]
    #[inline(always)]
    pub const fn tff(&self) -> &Tff {
        &self.tff
    }
    #[doc = "0x1c0 - receiver block dma register"]
    #[inline(always)]
    pub const fn rxdma(&self) -> &Rxdma {
        &self.rxdma
    }
    #[doc = "0x1c4 - reset receiver block dma register"]
    #[inline(always)]
    pub const fn rrxdma(&self) -> &Rrxdma {
        &self.rrxdma
    }
    #[doc = "0x1c8 - transmitter block dma register"]
    #[inline(always)]
    pub const fn txdma(&self) -> &Txdma {
        &self.txdma
    }
    #[doc = "0x1cc - reset transmitter block dma register"]
    #[inline(always)]
    pub const fn rtxdma(&self) -> &Rtxdma {
        &self.rtxdma
    }
    #[doc = "0x1f0 - component parameter register 2"]
    #[inline(always)]
    pub const fn i2s_comp_param_2(&self) -> &I2sCompParam2 {
        &self.i2s_comp_param_2
    }
    #[doc = "0x1f4 - component parameter register 1"]
    #[inline(always)]
    pub const fn i2s_comp_param_1(&self) -> &I2sCompParam1 {
        &self.i2s_comp_param_1
    }
    #[doc = "0x1f8 - component version register"]
    #[inline(always)]
    pub const fn i2s_comp_version(&self) -> &I2sCompVersion {
        &self.i2s_comp_version
    }
    #[doc = "0x1fc - component type register"]
    #[inline(always)]
    pub const fn i2s_comp_type(&self) -> &I2sCompType {
        &self.i2s_comp_type
    }
}
#[doc = "IER (rw) register accessor: enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "enable register"]
pub mod ier;
#[doc = "IRER (rw) register accessor: receiver block enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irer::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irer`] module"]
#[doc(alias = "IRER")]
pub type Irer = crate::Reg<irer::IrerSpec>;
#[doc = "receiver block enable register"]
pub mod irer;
#[doc = "ITER (rw) register accessor: transmitter block enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iter::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iter`] module"]
#[doc(alias = "ITER")]
pub type Iter = crate::Reg<iter::IterSpec>;
#[doc = "transmitter block enable register"]
pub mod iter;
#[doc = "CER (rw) register accessor: clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cer::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cer`] module"]
#[doc(alias = "CER")]
pub type Cer = crate::Reg<cer::CerSpec>;
#[doc = "clock enable register"]
pub mod cer;
#[doc = "CCR (rw) register accessor: clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "clock configuration register"]
pub mod ccr;
#[doc = "RXFFR (w) register accessor: receiver block FIFO reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxffr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxffr`] module"]
#[doc(alias = "RXFFR")]
pub type Rxffr = crate::Reg<rxffr::RxffrSpec>;
#[doc = "receiver block FIFO reset register"]
pub mod rxffr;
#[doc = "TXFFR (w) register accessor: transmitter block FIFO reset register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txffr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txffr`] module"]
#[doc(alias = "TXFFR")]
pub type Txffr = crate::Reg<txffr::TxffrSpec>;
#[doc = "transmitter block FIFO reset register"]
pub mod txffr;
#[doc = "LRBR_LTHR (rw) register accessor: right receive buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`lrbr_lthr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrbr_lthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrbr_lthr`] module"]
#[doc(alias = "LRBR_LTHR")]
pub type LrbrLthr = crate::Reg<lrbr_lthr::LrbrLthrSpec>;
#[doc = "right receive buffer register"]
pub mod lrbr_lthr;
#[doc = "RRBR_RTHR (rw) register accessor: right transmit holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrbr_rthr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrbr_rthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrbr_rthr`] module"]
#[doc(alias = "RRBR_RTHR")]
pub type RrbrRthr = crate::Reg<rrbr_rthr::RrbrRthrSpec>;
#[doc = "right transmit holding register"]
pub mod rrbr_rthr;
#[doc = "RER (rw) register accessor: receiver enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rer::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rer`] module"]
#[doc(alias = "RER")]
pub type Rer = crate::Reg<rer::RerSpec>;
#[doc = "receiver enable register"]
pub mod rer;
#[doc = "TER (rw) register accessor: transmitter enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ter::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter`] module"]
#[doc(alias = "TER")]
pub type Ter = crate::Reg<ter::TerSpec>;
#[doc = "transmitter enable register"]
pub mod ter;
#[doc = "RCR (rw) register accessor: receiver configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "receiver configuration register"]
pub mod rcr;
#[doc = "TCR (rw) register accessor: transmitter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "transmitter configuration register"]
pub mod tcr;
#[doc = "ISR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IMR (rw) register accessor: interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "interrupt mask register"]
pub mod imr;
#[doc = "ROR (r) register accessor: receiver overrun register\n\nYou can [`read`](crate::Reg::read) this register and get [`ror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ror`] module"]
#[doc(alias = "ROR")]
pub type Ror = crate::Reg<ror::RorSpec>;
#[doc = "receiver overrun register"]
pub mod ror;
#[doc = "TOR (r) register accessor: transmitter overrun register\n\nYou can [`read`](crate::Reg::read) this register and get [`tor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tor`] module"]
#[doc(alias = "TOR")]
pub type Tor = crate::Reg<tor::TorSpec>;
#[doc = "transmitter overrun register"]
pub mod tor;
#[doc = "RFCR (rw) register accessor: receiver FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcr`] module"]
#[doc(alias = "RFCR")]
pub type Rfcr = crate::Reg<rfcr::RfcrSpec>;
#[doc = "receiver FIFO configuration register"]
pub mod rfcr;
#[doc = "TFCR (rw) register accessor: transmitter FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfcr`] module"]
#[doc(alias = "TFCR")]
pub type Tfcr = crate::Reg<tfcr::TfcrSpec>;
#[doc = "transmitter FIFO configuration register"]
pub mod tfcr;
#[doc = "RFF (w) register accessor: receiver FIFO flush register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rff`] module"]
#[doc(alias = "RFF")]
pub type Rff = crate::Reg<rff::RffSpec>;
#[doc = "receiver FIFO flush register"]
pub mod rff;
#[doc = "TFF (w) register accessor: transmitter FIFO flush register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tff`] module"]
#[doc(alias = "TFF")]
pub type Tff = crate::Reg<tff::TffSpec>;
#[doc = "transmitter FIFO flush register"]
pub mod tff;
#[doc = "RXDMA (rw) register accessor: receiver block dma register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma`] module"]
#[doc(alias = "RXDMA")]
pub type Rxdma = crate::Reg<rxdma::RxdmaSpec>;
#[doc = "receiver block dma register"]
pub mod rxdma;
#[doc = "RRXDMA (w) register accessor: reset receiver block dma register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrxdma::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrxdma`] module"]
#[doc(alias = "RRXDMA")]
pub type Rrxdma = crate::Reg<rrxdma::RrxdmaSpec>;
#[doc = "reset receiver block dma register"]
pub mod rrxdma;
#[doc = "TXDMA (rw) register accessor: transmitter block dma register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdma`] module"]
#[doc(alias = "TXDMA")]
pub type Txdma = crate::Reg<txdma::TxdmaSpec>;
#[doc = "transmitter block dma register"]
pub mod txdma;
#[doc = "RTXDMA (w) register accessor: reset transmitter block dma register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtxdma::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtxdma`] module"]
#[doc(alias = "RTXDMA")]
pub type Rtxdma = crate::Reg<rtxdma::RtxdmaSpec>;
#[doc = "reset transmitter block dma register"]
pub mod rtxdma;
#[doc = "I2S_COMP_PARAM_2 (r) register accessor: component parameter register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_param_2`] module"]
#[doc(alias = "I2S_COMP_PARAM_2")]
pub type I2sCompParam2 = crate::Reg<i2s_comp_param_2::I2sCompParam2Spec>;
#[doc = "component parameter register 2"]
pub mod i2s_comp_param_2;
#[doc = "I2S_COMP_PARAM_1 (r) register accessor: component parameter register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_param_1`] module"]
#[doc(alias = "I2S_COMP_PARAM_1")]
pub type I2sCompParam1 = crate::Reg<i2s_comp_param_1::I2sCompParam1Spec>;
#[doc = "component parameter register 1"]
pub mod i2s_comp_param_1;
#[doc = "I2S_COMP_VERSION (r) register accessor: component version register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_version`] module"]
#[doc(alias = "I2S_COMP_VERSION")]
pub type I2sCompVersion = crate::Reg<i2s_comp_version::I2sCompVersionSpec>;
#[doc = "component version register"]
pub mod i2s_comp_version;
#[doc = "I2S_COMP_TYPE (r) register accessor: component type register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_type::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_comp_type`] module"]
#[doc(alias = "I2S_COMP_TYPE")]
pub type I2sCompType = crate::Reg<i2s_comp_type::I2sCompTypeSpec>;
#[doc = "component type register"]
pub mod i2s_comp_type;
