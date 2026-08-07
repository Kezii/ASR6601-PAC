#[repr(C)]
#[doc = "DMA Channel"]
#[doc(alias = "CH")]
pub struct Ch {
    sar_l: SarL,
    sar_h: SarH,
    dar_l: DarL,
    dar_h: DarH,
    llp_l: LlpL,
    llp_h: LlpH,
    ctl_l: CtlL,
    ctl_h: CtlH,
    sstat_l: SstatL,
    sstat_h: SstatH,
    dstat_l: DstatL,
    dstat_h: DstatH,
    sstatar_l: SstatarL,
    sstatar_h: SstatarH,
    dstatar_l: DstatarL,
    dstatar_h: DstatarH,
    cfg_l: CfgL,
    cfg_h: CfgH,
    sgr_l: SgrL,
    sgr_h: SgrH,
    dsr_l: DsrL,
    dsr_h: DsrH,
}
impl Ch {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sar_l(&self) -> &SarL {
        &self.sar_l
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn sar_h(&self) -> &SarH {
        &self.sar_h
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dar_l(&self) -> &DarL {
        &self.dar_l
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dar_h(&self) -> &DarH {
        &self.dar_h
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn llp_l(&self) -> &LlpL {
        &self.llp_l
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn llp_h(&self) -> &LlpH {
        &self.llp_h
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ctl_l(&self) -> &CtlL {
        &self.ctl_l
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ctl_h(&self) -> &CtlH {
        &self.ctl_h
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn sstat_l(&self) -> &SstatL {
        &self.sstat_l
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn sstat_h(&self) -> &SstatH {
        &self.sstat_h
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn dstat_l(&self) -> &DstatL {
        &self.dstat_l
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn dstat_h(&self) -> &DstatH {
        &self.dstat_h
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn sstatar_l(&self) -> &SstatarL {
        &self.sstatar_l
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sstatar_h(&self) -> &SstatarH {
        &self.sstatar_h
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn dstatar_l(&self) -> &DstatarL {
        &self.dstatar_l
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn dstatar_h(&self) -> &DstatarH {
        &self.dstatar_h
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn cfg_l(&self) -> &CfgL {
        &self.cfg_l
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn cfg_h(&self) -> &CfgH {
        &self.cfg_h
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn sgr_l(&self) -> &SgrL {
        &self.sgr_l
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sgr_h(&self) -> &SgrH {
        &self.sgr_h
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn dsr_l(&self) -> &DsrL {
        &self.dsr_l
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn dsr_h(&self) -> &DsrH {
        &self.dsr_h
    }
}
#[doc = "SAR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_l`] module"]
#[doc(alias = "SAR_L")]
pub type SarL = crate::Reg<sar_l::SarLSpec>;
#[doc = ""]
pub mod sar_l;
#[doc = "SAR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sar_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_h`] module"]
#[doc(alias = "SAR_H")]
pub type SarH = crate::Reg<sar_h::SarHSpec>;
#[doc = ""]
pub mod sar_h;
#[doc = "DAR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dar_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar_l`] module"]
#[doc(alias = "DAR_L")]
pub type DarL = crate::Reg<dar_l::DarLSpec>;
#[doc = ""]
pub mod dar_l;
#[doc = "DAR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dar_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar_h`] module"]
#[doc(alias = "DAR_H")]
pub type DarH = crate::Reg<dar_h::DarHSpec>;
#[doc = ""]
pub mod dar_h;
#[doc = "LLP_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`llp_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp_l`] module"]
#[doc(alias = "LLP_L")]
pub type LlpL = crate::Reg<llp_l::LlpLSpec>;
#[doc = ""]
pub mod llp_l;
#[doc = "LLP_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`llp_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp_h`] module"]
#[doc(alias = "LLP_H")]
pub type LlpH = crate::Reg<llp_h::LlpHSpec>;
#[doc = ""]
pub mod llp_h;
#[doc = "CTL_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctl_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_l`] module"]
#[doc(alias = "CTL_L")]
pub type CtlL = crate::Reg<ctl_l::CtlLSpec>;
#[doc = ""]
pub mod ctl_l;
#[doc = "CTL_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctl_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_h`] module"]
#[doc(alias = "CTL_H")]
pub type CtlH = crate::Reg<ctl_h::CtlHSpec>;
#[doc = ""]
pub mod ctl_h;
#[doc = "SSTAT_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sstat_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat_l`] module"]
#[doc(alias = "SSTAT_L")]
pub type SstatL = crate::Reg<sstat_l::SstatLSpec>;
#[doc = ""]
pub mod sstat_l;
#[doc = "SSTAT_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sstat_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstat_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat_h`] module"]
#[doc(alias = "SSTAT_H")]
pub type SstatH = crate::Reg<sstat_h::SstatHSpec>;
#[doc = ""]
pub mod sstat_h;
#[doc = "DSTAT_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dstat_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat_l`] module"]
#[doc(alias = "DSTAT_L")]
pub type DstatL = crate::Reg<dstat_l::DstatLSpec>;
#[doc = ""]
pub mod dstat_l;
#[doc = "DSTAT_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dstat_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstat_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat_h`] module"]
#[doc(alias = "DSTAT_H")]
pub type DstatH = crate::Reg<dstat_h::DstatHSpec>;
#[doc = ""]
pub mod dstat_h;
#[doc = "SSTATAR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar_l`] module"]
#[doc(alias = "SSTATAR_L")]
pub type SstatarL = crate::Reg<sstatar_l::SstatarLSpec>;
#[doc = ""]
pub mod sstatar_l;
#[doc = "SSTATAR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstatar_h`] module"]
#[doc(alias = "SSTATAR_H")]
pub type SstatarH = crate::Reg<sstatar_h::SstatarHSpec>;
#[doc = ""]
pub mod sstatar_h;
#[doc = "DSTATAR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar_l`] module"]
#[doc(alias = "DSTATAR_L")]
pub type DstatarL = crate::Reg<dstatar_l::DstatarLSpec>;
#[doc = ""]
pub mod dstatar_l;
#[doc = "DSTATAR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatar_h`] module"]
#[doc(alias = "DSTATAR_H")]
pub type DstatarH = crate::Reg<dstatar_h::DstatarHSpec>;
#[doc = ""]
pub mod dstatar_h;
#[doc = "CFG_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_l`] module"]
#[doc(alias = "CFG_L")]
pub type CfgL = crate::Reg<cfg_l::CfgLSpec>;
#[doc = ""]
pub mod cfg_l;
#[doc = "CFG_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_h`] module"]
#[doc(alias = "CFG_H")]
pub type CfgH = crate::Reg<cfg_h::CfgHSpec>;
#[doc = ""]
pub mod cfg_h;
#[doc = "SGR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sgr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgr_l`] module"]
#[doc(alias = "SGR_L")]
pub type SgrL = crate::Reg<sgr_l::SgrLSpec>;
#[doc = ""]
pub mod sgr_l;
#[doc = "SGR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sgr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgr_h`] module"]
#[doc(alias = "SGR_H")]
pub type SgrH = crate::Reg<sgr_h::SgrHSpec>;
#[doc = ""]
pub mod sgr_h;
#[doc = "DSR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_l`] module"]
#[doc(alias = "DSR_L")]
pub type DsrL = crate::Reg<dsr_l::DsrLSpec>;
#[doc = ""]
pub mod dsr_l;
#[doc = "DSR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_h`] module"]
#[doc(alias = "DSR_H")]
pub type DsrH = crate::Reg<dsr_h::DsrHSpec>;
#[doc = ""]
pub mod dsr_h;
