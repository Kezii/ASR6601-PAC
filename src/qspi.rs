#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    qspi_cr: QspiCr,
    qspi_dcr: QspiDcr,
    qspi_sr: QspiSr,
    qspi_fcr: QspiFcr,
    qspi_dlr: QspiDlr,
    qspi_ccr: QspiCcr,
    qspi_ar: QspiAr,
    qspi_abr: QspiAbr,
    qspi_dr: QspiDr,
    qspi_psmkr: QspiPsmkr,
    qspi_psmar: QspiPsmar,
    qspi_pir: QspiPir,
    qspi_tor: QspiTor,
    _reserved13: [u8; 0x4c],
    qspi_hit0r: QspiHit0r,
    qspi_hit1r: QspiHit1r,
    qspi_mir: QspiMir,
    qspi_cfgr: QspiCfgr,
    sbus_start: SbusStart,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn qspi_cr(&self) -> &QspiCr {
        &self.qspi_cr
    }
    #[doc = "0x04 - device configuration register"]
    #[inline(always)]
    pub const fn qspi_dcr(&self) -> &QspiDcr {
        &self.qspi_dcr
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn qspi_sr(&self) -> &QspiSr {
        &self.qspi_sr
    }
    #[doc = "0x0c - flag clear register"]
    #[inline(always)]
    pub const fn qspi_fcr(&self) -> &QspiFcr {
        &self.qspi_fcr
    }
    #[doc = "0x10 - data length register"]
    #[inline(always)]
    pub const fn qspi_dlr(&self) -> &QspiDlr {
        &self.qspi_dlr
    }
    #[doc = "0x14 - communication configuration register"]
    #[inline(always)]
    pub const fn qspi_ccr(&self) -> &QspiCcr {
        &self.qspi_ccr
    }
    #[doc = "0x18 - address register"]
    #[inline(always)]
    pub const fn qspi_ar(&self) -> &QspiAr {
        &self.qspi_ar
    }
    #[doc = "0x1c - alternate byte register"]
    #[inline(always)]
    pub const fn qspi_abr(&self) -> &QspiAbr {
        &self.qspi_abr
    }
    #[doc = "0x20 - data register"]
    #[inline(always)]
    pub const fn qspi_dr(&self) -> &QspiDr {
        &self.qspi_dr
    }
    #[doc = "0x24 - polling status mask register"]
    #[inline(always)]
    pub const fn qspi_psmkr(&self) -> &QspiPsmkr {
        &self.qspi_psmkr
    }
    #[doc = "0x28 - polling status match register"]
    #[inline(always)]
    pub const fn qspi_psmar(&self) -> &QspiPsmar {
        &self.qspi_psmar
    }
    #[doc = "0x2c - polling interval register"]
    #[inline(always)]
    pub const fn qspi_pir(&self) -> &QspiPir {
        &self.qspi_pir
    }
    #[doc = "0x30 - timeout register"]
    #[inline(always)]
    pub const fn qspi_tor(&self) -> &QspiTor {
        &self.qspi_tor
    }
    #[doc = "0x80 - hit0 times accumulator register"]
    #[inline(always)]
    pub const fn qspi_hit0r(&self) -> &QspiHit0r {
        &self.qspi_hit0r
    }
    #[doc = "0x84 - hit1 times accumulator register"]
    #[inline(always)]
    pub const fn qspi_hit1r(&self) -> &QspiHit1r {
        &self.qspi_hit1r
    }
    #[doc = "0x88 - miss times accumulator register"]
    #[inline(always)]
    pub const fn qspi_mir(&self) -> &QspiMir {
        &self.qspi_mir
    }
    #[doc = "0x8c - configuration register"]
    #[inline(always)]
    pub const fn qspi_cfgr(&self) -> &QspiCfgr {
        &self.qspi_cfgr
    }
    #[doc = "0x90 - start register"]
    #[inline(always)]
    pub const fn sbus_start(&self) -> &SbusStart {
        &self.sbus_start
    }
}
#[doc = "QSPI_CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_cr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_cr`] module"]
#[doc(alias = "QSPI_CR")]
pub type QspiCr = crate::Reg<qspi_cr::QspiCrSpec>;
#[doc = "control register"]
pub mod qspi_cr;
#[doc = "QSPI_DCR (rw) register accessor: device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_dcr`] module"]
#[doc(alias = "QSPI_DCR")]
pub type QspiDcr = crate::Reg<qspi_dcr::QspiDcrSpec>;
#[doc = "device configuration register"]
pub mod qspi_dcr;
#[doc = "QSPI_SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_sr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_sr`] module"]
#[doc(alias = "QSPI_SR")]
pub type QspiSr = crate::Reg<qspi_sr::QspiSrSpec>;
#[doc = "status register"]
pub mod qspi_sr;
#[doc = "QSPI_FCR (rw) register accessor: flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_fcr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_fcr`] module"]
#[doc(alias = "QSPI_FCR")]
pub type QspiFcr = crate::Reg<qspi_fcr::QspiFcrSpec>;
#[doc = "flag clear register"]
pub mod qspi_fcr;
#[doc = "QSPI_DLR (rw) register accessor: data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dlr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_dlr`] module"]
#[doc(alias = "QSPI_DLR")]
pub type QspiDlr = crate::Reg<qspi_dlr::QspiDlrSpec>;
#[doc = "data length register"]
pub mod qspi_dlr;
#[doc = "QSPI_CCR (rw) register accessor: communication configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_ccr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_ccr`] module"]
#[doc(alias = "QSPI_CCR")]
pub type QspiCcr = crate::Reg<qspi_ccr::QspiCcrSpec>;
#[doc = "communication configuration register"]
pub mod qspi_ccr;
#[doc = "QSPI_AR (rw) register accessor: address register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_ar::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_ar`] module"]
#[doc(alias = "QSPI_AR")]
pub type QspiAr = crate::Reg<qspi_ar::QspiArSpec>;
#[doc = "address register"]
pub mod qspi_ar;
#[doc = "QSPI_ABR (rw) register accessor: alternate byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_abr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_abr`] module"]
#[doc(alias = "QSPI_ABR")]
pub type QspiAbr = crate::Reg<qspi_abr::QspiAbrSpec>;
#[doc = "alternate byte register"]
pub mod qspi_abr;
#[doc = "QSPI_DR (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_dr`] module"]
#[doc(alias = "QSPI_DR")]
pub type QspiDr = crate::Reg<qspi_dr::QspiDrSpec>;
#[doc = "data register"]
pub mod qspi_dr;
#[doc = "QSPI_PSMKR (rw) register accessor: polling status mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_psmkr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_psmkr`] module"]
#[doc(alias = "QSPI_PSMKR")]
pub type QspiPsmkr = crate::Reg<qspi_psmkr::QspiPsmkrSpec>;
#[doc = "polling status mask register"]
pub mod qspi_psmkr;
#[doc = "QSPI_PSMAR (rw) register accessor: polling status match register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_psmar::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_psmar`] module"]
#[doc(alias = "QSPI_PSMAR")]
pub type QspiPsmar = crate::Reg<qspi_psmar::QspiPsmarSpec>;
#[doc = "polling status match register"]
pub mod qspi_psmar;
#[doc = "QSPI_PIR (rw) register accessor: polling interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_pir::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_pir`] module"]
#[doc(alias = "QSPI_PIR")]
pub type QspiPir = crate::Reg<qspi_pir::QspiPirSpec>;
#[doc = "polling interval register"]
pub mod qspi_pir;
#[doc = "QSPI_TOR (rw) register accessor: timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_tor::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_tor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_tor`] module"]
#[doc(alias = "QSPI_TOR")]
pub type QspiTor = crate::Reg<qspi_tor::QspiTorSpec>;
#[doc = "timeout register"]
pub mod qspi_tor;
#[doc = "QSPI_HIT0R (rw) register accessor: hit0 times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_hit0r::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_hit0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_hit0r`] module"]
#[doc(alias = "QSPI_HIT0R")]
pub type QspiHit0r = crate::Reg<qspi_hit0r::QspiHit0rSpec>;
#[doc = "hit0 times accumulator register"]
pub mod qspi_hit0r;
#[doc = "QSPI_HIT1R (rw) register accessor: hit1 times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_hit1r::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_hit1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_hit1r`] module"]
#[doc(alias = "QSPI_HIT1R")]
pub type QspiHit1r = crate::Reg<qspi_hit1r::QspiHit1rSpec>;
#[doc = "hit1 times accumulator register"]
pub mod qspi_hit1r;
#[doc = "QSPI_MIR (rw) register accessor: miss times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_mir::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_mir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_mir`] module"]
#[doc(alias = "QSPI_MIR")]
pub type QspiMir = crate::Reg<qspi_mir::QspiMirSpec>;
#[doc = "miss times accumulator register"]
pub mod qspi_mir;
#[doc = "QSPI_CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_cfgr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_cfgr`] module"]
#[doc(alias = "QSPI_CFGR")]
pub type QspiCfgr = crate::Reg<qspi_cfgr::QspiCfgrSpec>;
#[doc = "configuration register"]
pub mod qspi_cfgr;
#[doc = "SBUS_START (rw) register accessor: start register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbus_start::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbus_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbus_start`] module"]
#[doc(alias = "SBUS_START")]
pub type SbusStart = crate::Reg<sbus_start::SbusStartSpec>;
#[doc = "start register"]
pub mod sbus_start;
