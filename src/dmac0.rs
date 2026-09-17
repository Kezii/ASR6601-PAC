#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sar0: Sar0,
    dar0: Dar0,
    llp0: Llp0,
    ctl0: Ctl0,
    _reserved4: [u8; 0x20],
    cfg0: Cfg0,
    _reserved5: [u8; 0x10],
    sar1: Sar1,
    dar1: Dar1,
    llp1: Llp1,
    ctl1: Ctl1,
    _reserved9: [u8; 0x20],
    cfg1: Cfg1,
    _reserved10: [u8; 0x10],
    sar2: Sar2,
    dar2: Dar2,
    llp2: Llp2,
    ctl2: Ctl2,
    _reserved14: [u8; 0x20],
    cfg2: Cfg2,
    _reserved15: [u8; 0x10],
    sar3: Sar3,
    dar3: Dar3,
    llp3: Llp3,
    ctl3: Ctl3,
    _reserved19: [u8; 0x20],
    cfg3: Cfg3,
    _reserved20: [u8; 0x0198],
    status_tfr: StatusTfr,
    status_block: StatusBlock,
    status_src_tran: StatusSrcTran,
    status_dst_tran: StatusDstTran,
    status_err: StatusErr,
    mask_tfr: MaskTfr,
    mask_block: MaskBlock,
    mask_src_tran: MaskSrcTran,
    mask_dst_tran: MaskDstTran,
    mask_err: MaskErr,
    clear_tfr: ClearTfr,
    clear_block: ClearBlock,
    clear_src_tran: ClearSrcTran,
    clear_dst_tran: ClearDstTran,
    clear_err: ClearErr,
    _reserved35: [u8; 0x38],
    dmacfgreg: Dmacfgreg,
    chenreg: Chenreg,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - source address register"]
    #[inline(always)]
    pub const fn sar0(&self) -> &Sar0 {
        &self.sar0
    }
    #[doc = "0x08..0x10 - destination address register"]
    #[inline(always)]
    pub const fn dar0(&self) -> &Dar0 {
        &self.dar0
    }
    #[doc = "0x10..0x18 - linked list pointer register"]
    #[inline(always)]
    pub const fn llp0(&self) -> &Llp0 {
        &self.llp0
    }
    #[doc = "0x18..0x20 - channel control register"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x40..0x48 - channel configuration register"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x58..0x60 - source address register"]
    #[inline(always)]
    pub const fn sar1(&self) -> &Sar1 {
        &self.sar1
    }
    #[doc = "0x60..0x68 - destination address register"]
    #[inline(always)]
    pub const fn dar1(&self) -> &Dar1 {
        &self.dar1
    }
    #[doc = "0x68..0x70 - linked list pointer register"]
    #[inline(always)]
    pub const fn llp1(&self) -> &Llp1 {
        &self.llp1
    }
    #[doc = "0x70..0x78 - channel control register"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x98..0xa0 - channel configuration register"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0xb0..0xb8 - source address register"]
    #[inline(always)]
    pub const fn sar2(&self) -> &Sar2 {
        &self.sar2
    }
    #[doc = "0xb8..0xc0 - destination address register"]
    #[inline(always)]
    pub const fn dar2(&self) -> &Dar2 {
        &self.dar2
    }
    #[doc = "0xc0..0xc8 - linked list pointer register"]
    #[inline(always)]
    pub const fn llp2(&self) -> &Llp2 {
        &self.llp2
    }
    #[doc = "0xc8..0xd0 - channel control register"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0xf0..0xf8 - channel configuration register"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x108..0x110 - source address register"]
    #[inline(always)]
    pub const fn sar3(&self) -> &Sar3 {
        &self.sar3
    }
    #[doc = "0x110..0x118 - destination address register"]
    #[inline(always)]
    pub const fn dar3(&self) -> &Dar3 {
        &self.dar3
    }
    #[doc = "0x118..0x120 - linked list pointer register"]
    #[inline(always)]
    pub const fn llp3(&self) -> &Llp3 {
        &self.llp3
    }
    #[doc = "0x120..0x128 - channel control register"]
    #[inline(always)]
    pub const fn ctl3(&self) -> &Ctl3 {
        &self.ctl3
    }
    #[doc = "0x148..0x150 - channel configuration register"]
    #[inline(always)]
    pub const fn cfg3(&self) -> &Cfg3 {
        &self.cfg3
    }
    #[doc = "0x2e8..0x2f0 - DMA transfer complete interrupt status register"]
    #[inline(always)]
    pub const fn status_tfr(&self) -> &StatusTfr {
        &self.status_tfr
    }
    #[doc = "0x2f0..0x2f8 - DMA block transfer complete interrupt status register"]
    #[inline(always)]
    pub const fn status_block(&self) -> &StatusBlock {
        &self.status_block
    }
    #[doc = "0x2f8..0x300 - DMA source processing completed interrupt status register"]
    #[inline(always)]
    pub const fn status_src_tran(&self) -> &StatusSrcTran {
        &self.status_src_tran
    }
    #[doc = "0x300..0x308 - DMA destination processing completed interrupt status register"]
    #[inline(always)]
    pub const fn status_dst_tran(&self) -> &StatusDstTran {
        &self.status_dst_tran
    }
    #[doc = "0x308..0x310 - DMA transmission error status register"]
    #[inline(always)]
    pub const fn status_err(&self) -> &StatusErr {
        &self.status_err
    }
    #[doc = "0x310..0x318 - DMA transfer complete interrupt enable register"]
    #[inline(always)]
    pub const fn mask_tfr(&self) -> &MaskTfr {
        &self.mask_tfr
    }
    #[doc = "0x318..0x320 - DMA block transfer complete interrupt enable register"]
    #[inline(always)]
    pub const fn mask_block(&self) -> &MaskBlock {
        &self.mask_block
    }
    #[doc = "0x320..0x328 - DMA source processing completed interrupt enable register"]
    #[inline(always)]
    pub const fn mask_src_tran(&self) -> &MaskSrcTran {
        &self.mask_src_tran
    }
    #[doc = "0x328..0x330 - DMA destination processing completed interrupt enable register"]
    #[inline(always)]
    pub const fn mask_dst_tran(&self) -> &MaskDstTran {
        &self.mask_dst_tran
    }
    #[doc = "0x330..0x338 - DMA transmission error interrupt enable register"]
    #[inline(always)]
    pub const fn mask_err(&self) -> &MaskErr {
        &self.mask_err
    }
    #[doc = "0x338..0x340 - DMA transfer completion status clear register"]
    #[inline(always)]
    pub const fn clear_tfr(&self) -> &ClearTfr {
        &self.clear_tfr
    }
    #[doc = "0x340..0x348 - DMA block transfer completion status clear register"]
    #[inline(always)]
    pub const fn clear_block(&self) -> &ClearBlock {
        &self.clear_block
    }
    #[doc = "0x348..0x350 - DMA source transfer completion status clear register"]
    #[inline(always)]
    pub const fn clear_src_tran(&self) -> &ClearSrcTran {
        &self.clear_src_tran
    }
    #[doc = "0x350..0x358 - DMA destination transfer completion status clear register"]
    #[inline(always)]
    pub const fn clear_dst_tran(&self) -> &ClearDstTran {
        &self.clear_dst_tran
    }
    #[doc = "0x358..0x360 - DMA transfer error status clear register"]
    #[inline(always)]
    pub const fn clear_err(&self) -> &ClearErr {
        &self.clear_err
    }
    #[doc = "0x398..0x3a0 - DMA enable register"]
    #[inline(always)]
    pub const fn dmacfgreg(&self) -> &Dmacfgreg {
        &self.dmacfgreg
    }
    #[doc = "0x3a0..0x3a8 - DMA channel enable register"]
    #[inline(always)]
    pub const fn chenreg(&self) -> &Chenreg {
        &self.chenreg
    }
}
#[doc = "SAR0 (rw) register accessor: source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar0`] module"]
#[doc(alias = "SAR0")]
pub type Sar0 = crate::Reg<sar0::Sar0Spec>;
#[doc = "source address register"]
pub mod sar0;
#[doc = "DAR0 (rw) register accessor: destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar0`] module"]
#[doc(alias = "DAR0")]
pub type Dar0 = crate::Reg<dar0::Dar0Spec>;
#[doc = "destination address register"]
pub mod dar0;
#[doc = "LLP0 (rw) register accessor: linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp0`] module"]
#[doc(alias = "LLP0")]
pub type Llp0 = crate::Reg<llp0::Llp0Spec>;
#[doc = "linked list pointer register"]
pub mod llp0;
#[doc = "CTL0 (rw) register accessor: channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "channel control register"]
pub mod ctl0;
#[doc = "CFG0 (rw) register accessor: channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "channel configuration register"]
pub mod cfg0;
#[doc = "SAR1 (rw) register accessor: source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1`] module"]
#[doc(alias = "SAR1")]
pub type Sar1 = crate::Reg<sar1::Sar1Spec>;
#[doc = "source address register"]
pub mod sar1;
#[doc = "DAR1 (rw) register accessor: destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar1`] module"]
#[doc(alias = "DAR1")]
pub type Dar1 = crate::Reg<dar1::Dar1Spec>;
#[doc = "destination address register"]
pub mod dar1;
#[doc = "LLP1 (rw) register accessor: linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp1`] module"]
#[doc(alias = "LLP1")]
pub type Llp1 = crate::Reg<llp1::Llp1Spec>;
#[doc = "linked list pointer register"]
pub mod llp1;
#[doc = "CTL1 (rw) register accessor: channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "channel control register"]
pub mod ctl1;
#[doc = "CFG1 (rw) register accessor: channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "channel configuration register"]
pub mod cfg1;
#[doc = "SAR2 (rw) register accessor: source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2`] module"]
#[doc(alias = "SAR2")]
pub type Sar2 = crate::Reg<sar2::Sar2Spec>;
#[doc = "source address register"]
pub mod sar2;
#[doc = "DAR2 (rw) register accessor: destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar2`] module"]
#[doc(alias = "DAR2")]
pub type Dar2 = crate::Reg<dar2::Dar2Spec>;
#[doc = "destination address register"]
pub mod dar2;
#[doc = "LLP2 (rw) register accessor: linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp2`] module"]
#[doc(alias = "LLP2")]
pub type Llp2 = crate::Reg<llp2::Llp2Spec>;
#[doc = "linked list pointer register"]
pub mod llp2;
#[doc = "CTL2 (rw) register accessor: channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`] module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "channel control register"]
pub mod ctl2;
#[doc = "CFG2 (rw) register accessor: channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`] module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "channel configuration register"]
pub mod cfg2;
#[doc = "SAR3 (rw) register accessor: source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar3`] module"]
#[doc(alias = "SAR3")]
pub type Sar3 = crate::Reg<sar3::Sar3Spec>;
#[doc = "source address register"]
pub mod sar3;
#[doc = "DAR3 (rw) register accessor: destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar3`] module"]
#[doc(alias = "DAR3")]
pub type Dar3 = crate::Reg<dar3::Dar3Spec>;
#[doc = "destination address register"]
pub mod dar3;
#[doc = "LLP3 (rw) register accessor: linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp3`] module"]
#[doc(alias = "LLP3")]
pub type Llp3 = crate::Reg<llp3::Llp3Spec>;
#[doc = "linked list pointer register"]
pub mod llp3;
#[doc = "CTL3 (rw) register accessor: channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl3`] module"]
#[doc(alias = "CTL3")]
pub type Ctl3 = crate::Reg<ctl3::Ctl3Spec>;
#[doc = "channel control register"]
pub mod ctl3;
#[doc = "CFG3 (rw) register accessor: channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg3`] module"]
#[doc(alias = "CFG3")]
pub type Cfg3 = crate::Reg<cfg3::Cfg3Spec>;
#[doc = "channel configuration register"]
pub mod cfg3;
#[doc = "STATUS_TFR (r) register accessor: DMA transfer complete interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_tfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_tfr`] module"]
#[doc(alias = "STATUS_TFR")]
pub type StatusTfr = crate::Reg<status_tfr::StatusTfrSpec>;
#[doc = "DMA transfer complete interrupt status register"]
pub mod status_tfr;
#[doc = "STATUS_BLOCK (r) register accessor: DMA block transfer complete interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_block::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_block`] module"]
#[doc(alias = "STATUS_BLOCK")]
pub type StatusBlock = crate::Reg<status_block::StatusBlockSpec>;
#[doc = "DMA block transfer complete interrupt status register"]
pub mod status_block;
#[doc = "STATUS_SRC_TRAN (r) register accessor: DMA source processing completed interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_src_tran::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_src_tran`] module"]
#[doc(alias = "STATUS_SRC_TRAN")]
pub type StatusSrcTran = crate::Reg<status_src_tran::StatusSrcTranSpec>;
#[doc = "DMA source processing completed interrupt status register"]
pub mod status_src_tran;
#[doc = "STATUS_DST_TRAN (r) register accessor: DMA destination processing completed interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_dst_tran::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_dst_tran`] module"]
#[doc(alias = "STATUS_DST_TRAN")]
pub type StatusDstTran = crate::Reg<status_dst_tran::StatusDstTranSpec>;
#[doc = "DMA destination processing completed interrupt status register"]
pub mod status_dst_tran;
#[doc = "STATUS_ERR (r) register accessor: DMA transmission error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_err`] module"]
#[doc(alias = "STATUS_ERR")]
pub type StatusErr = crate::Reg<status_err::StatusErrSpec>;
#[doc = "DMA transmission error status register"]
pub mod status_err;
#[doc = "MASK_TFR (rw) register accessor: DMA transfer complete interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_tfr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_tfr`] module"]
#[doc(alias = "MASK_TFR")]
pub type MaskTfr = crate::Reg<mask_tfr::MaskTfrSpec>;
#[doc = "DMA transfer complete interrupt enable register"]
pub mod mask_tfr;
#[doc = "MASK_BLOCK (rw) register accessor: DMA block transfer complete interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_block::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_block::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_block`] module"]
#[doc(alias = "MASK_BLOCK")]
pub type MaskBlock = crate::Reg<mask_block::MaskBlockSpec>;
#[doc = "DMA block transfer complete interrupt enable register"]
pub mod mask_block;
#[doc = "MASK_SRC_TRAN (rw) register accessor: DMA source processing completed interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_src_tran::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_src_tran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_src_tran`] module"]
#[doc(alias = "MASK_SRC_TRAN")]
pub type MaskSrcTran = crate::Reg<mask_src_tran::MaskSrcTranSpec>;
#[doc = "DMA source processing completed interrupt enable register"]
pub mod mask_src_tran;
#[doc = "MASK_DST_TRAN (rw) register accessor: DMA destination processing completed interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_dst_tran::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_dst_tran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_dst_tran`] module"]
#[doc(alias = "MASK_DST_TRAN")]
pub type MaskDstTran = crate::Reg<mask_dst_tran::MaskDstTranSpec>;
#[doc = "DMA destination processing completed interrupt enable register"]
pub mod mask_dst_tran;
#[doc = "MASK_ERR (rw) register accessor: DMA transmission error interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_err::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_err`] module"]
#[doc(alias = "MASK_ERR")]
pub type MaskErr = crate::Reg<mask_err::MaskErrSpec>;
#[doc = "DMA transmission error interrupt enable register"]
pub mod mask_err;
#[doc = "CLEAR_TFR (w) register accessor: DMA transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_tfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_tfr`] module"]
#[doc(alias = "CLEAR_TFR")]
pub type ClearTfr = crate::Reg<clear_tfr::ClearTfrSpec>;
#[doc = "DMA transfer completion status clear register"]
pub mod clear_tfr;
#[doc = "CLEAR_BLOCK (w) register accessor: DMA block transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_block`] module"]
#[doc(alias = "CLEAR_BLOCK")]
pub type ClearBlock = crate::Reg<clear_block::ClearBlockSpec>;
#[doc = "DMA block transfer completion status clear register"]
pub mod clear_block;
#[doc = "CLEAR_SRC_TRAN (w) register accessor: DMA source transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_src_tran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_src_tran`] module"]
#[doc(alias = "CLEAR_SRC_TRAN")]
pub type ClearSrcTran = crate::Reg<clear_src_tran::ClearSrcTranSpec>;
#[doc = "DMA source transfer completion status clear register"]
pub mod clear_src_tran;
#[doc = "CLEAR_DST_TRAN (w) register accessor: DMA destination transfer completion status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_dst_tran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_dst_tran`] module"]
#[doc(alias = "CLEAR_DST_TRAN")]
pub type ClearDstTran = crate::Reg<clear_dst_tran::ClearDstTranSpec>;
#[doc = "DMA destination transfer completion status clear register"]
pub mod clear_dst_tran;
#[doc = "CLEAR_ERR (w) register accessor: DMA transfer error status clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_err`] module"]
#[doc(alias = "CLEAR_ERR")]
pub type ClearErr = crate::Reg<clear_err::ClearErrSpec>;
#[doc = "DMA transfer error status clear register"]
pub mod clear_err;
#[doc = "DMACFGREG (rw) register accessor: DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgreg`] module"]
#[doc(alias = "DMACFGREG")]
pub type Dmacfgreg = crate::Reg<dmacfgreg::DmacfgregSpec>;
#[doc = "DMA enable register"]
pub mod dmacfgreg;
#[doc = "CHENREG (rw) register accessor: DMA channel enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenreg`] module"]
#[doc(alias = "CHENREG")]
pub type Chenreg = crate::Reg<chenreg::ChenregSpec>;
#[doc = "DMA channel enable register"]
pub mod chenreg;
