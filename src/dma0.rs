#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [Ch; 4],
    _reserved1: [u8; 0x0160],
    raw_tfr_l: RawTfrL,
    raw_tfr_h: RawTfrH,
    raw_block_l: RawBlockL,
    raw_block_h: RawBlockH,
    _reserved5: [u8; 0x18],
    status_tfr_l: StatusTfrL,
    status_tfr_h: StatusTfrH,
    status_block_l: StatusBlockL,
    status_block_h: StatusBlockH,
    _reserved9: [u8; 0x18],
    mask_tfr_l: MaskTfrL,
    mask_tfr_h: MaskTfrH,
    mask_block_l: MaskBlockL,
    mask_block_h: MaskBlockH,
    _reserved13: [u8; 0x18],
    clear_tfr_l: ClearTfrL,
    clear_tfr_h: ClearTfrH,
    clear_block_l: ClearBlockL,
    clear_block_h: ClearBlockH,
    clear_src_tran_l: ClearSrcTranL,
    clear_src_tran_h: ClearSrcTranH,
    clear_dst_tran_l: ClearDstTranL,
    clear_dst_tran_h: ClearDstTranH,
    clear_err_l: ClearErrL,
    clear_err_h: ClearErrH,
    status_int: StatusInt,
    _reserved24: [u8; 0x04],
    req_src: ReqSrc,
    _reserved25: [u8; 0x04],
    req_dst: ReqDst,
    _reserved26: [u8; 0x04],
    sgl_req_src: SglReqSrc,
    _reserved27: [u8; 0x04],
    sgl_req_dst: SglReqDst,
    _reserved28: [u8; 0x14],
    dmacfgreg_l: DmacfgregL,
    dmacfgreg_h: DmacfgregH,
    chenreg_l: ChenregL,
    chenreg_h: ChenregH,
    _reserved32: [u8; 0x38],
    dma_comp_params_3_l: DmaCompParams3L,
    dma_comp_params_3_h: DmaCompParams3H,
    dma_comp_params_2_l: DmaCompParams2L,
    dma_comp_params_2_h: DmaCompParams2H,
    dma_comp_params_1_l: DmaCompParams1L,
    dma_comp_params_1_h: DmaCompParams1H,
}
impl RegisterBlock {
    #[doc = "0x00..0x160 - DMA Channel"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x160 - DMA Channel"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn raw_tfr_l(&self) -> &RawTfrL {
        &self.raw_tfr_l
    }
    #[doc = "0x2c4 - "]
    #[inline(always)]
    pub const fn raw_tfr_h(&self) -> &RawTfrH {
        &self.raw_tfr_h
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn raw_block_l(&self) -> &RawBlockL {
        &self.raw_block_l
    }
    #[doc = "0x2cc - "]
    #[inline(always)]
    pub const fn raw_block_h(&self) -> &RawBlockH {
        &self.raw_block_h
    }
    #[doc = "0x2e8 - "]
    #[inline(always)]
    pub const fn status_tfr_l(&self) -> &StatusTfrL {
        &self.status_tfr_l
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn status_tfr_h(&self) -> &StatusTfrH {
        &self.status_tfr_h
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn status_block_l(&self) -> &StatusBlockL {
        &self.status_block_l
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn status_block_h(&self) -> &StatusBlockH {
        &self.status_block_h
    }
    #[doc = "0x310 - "]
    #[inline(always)]
    pub const fn mask_tfr_l(&self) -> &MaskTfrL {
        &self.mask_tfr_l
    }
    #[doc = "0x314 - "]
    #[inline(always)]
    pub const fn mask_tfr_h(&self) -> &MaskTfrH {
        &self.mask_tfr_h
    }
    #[doc = "0x318 - "]
    #[inline(always)]
    pub const fn mask_block_l(&self) -> &MaskBlockL {
        &self.mask_block_l
    }
    #[doc = "0x31c - "]
    #[inline(always)]
    pub const fn mask_block_h(&self) -> &MaskBlockH {
        &self.mask_block_h
    }
    #[doc = "0x338 - "]
    #[inline(always)]
    pub const fn clear_tfr_l(&self) -> &ClearTfrL {
        &self.clear_tfr_l
    }
    #[doc = "0x33c - "]
    #[inline(always)]
    pub const fn clear_tfr_h(&self) -> &ClearTfrH {
        &self.clear_tfr_h
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn clear_block_l(&self) -> &ClearBlockL {
        &self.clear_block_l
    }
    #[doc = "0x344 - "]
    #[inline(always)]
    pub const fn clear_block_h(&self) -> &ClearBlockH {
        &self.clear_block_h
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn clear_src_tran_l(&self) -> &ClearSrcTranL {
        &self.clear_src_tran_l
    }
    #[doc = "0x34c - "]
    #[inline(always)]
    pub const fn clear_src_tran_h(&self) -> &ClearSrcTranH {
        &self.clear_src_tran_h
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn clear_dst_tran_l(&self) -> &ClearDstTranL {
        &self.clear_dst_tran_l
    }
    #[doc = "0x354 - "]
    #[inline(always)]
    pub const fn clear_dst_tran_h(&self) -> &ClearDstTranH {
        &self.clear_dst_tran_h
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn clear_err_l(&self) -> &ClearErrL {
        &self.clear_err_l
    }
    #[doc = "0x35c - "]
    #[inline(always)]
    pub const fn clear_err_h(&self) -> &ClearErrH {
        &self.clear_err_h
    }
    #[doc = "0x360 - "]
    #[inline(always)]
    pub const fn status_int(&self) -> &StatusInt {
        &self.status_int
    }
    #[doc = "0x368 - "]
    #[inline(always)]
    pub const fn req_src(&self) -> &ReqSrc {
        &self.req_src
    }
    #[doc = "0x370 - "]
    #[inline(always)]
    pub const fn req_dst(&self) -> &ReqDst {
        &self.req_dst
    }
    #[doc = "0x378 - "]
    #[inline(always)]
    pub const fn sgl_req_src(&self) -> &SglReqSrc {
        &self.sgl_req_src
    }
    #[doc = "0x380 - "]
    #[inline(always)]
    pub const fn sgl_req_dst(&self) -> &SglReqDst {
        &self.sgl_req_dst
    }
    #[doc = "0x398 - "]
    #[inline(always)]
    pub const fn dmacfgreg_l(&self) -> &DmacfgregL {
        &self.dmacfgreg_l
    }
    #[doc = "0x39c - "]
    #[inline(always)]
    pub const fn dmacfgreg_h(&self) -> &DmacfgregH {
        &self.dmacfgreg_h
    }
    #[doc = "0x3a0 - "]
    #[inline(always)]
    pub const fn chenreg_l(&self) -> &ChenregL {
        &self.chenreg_l
    }
    #[doc = "0x3a4 - "]
    #[inline(always)]
    pub const fn chenreg_h(&self) -> &ChenregH {
        &self.chenreg_h
    }
    #[doc = "0x3e0 - "]
    #[inline(always)]
    pub const fn dma_comp_params_3_l(&self) -> &DmaCompParams3L {
        &self.dma_comp_params_3_l
    }
    #[doc = "0x3e4 - "]
    #[inline(always)]
    pub const fn dma_comp_params_3_h(&self) -> &DmaCompParams3H {
        &self.dma_comp_params_3_h
    }
    #[doc = "0x3e8 - "]
    #[inline(always)]
    pub const fn dma_comp_params_2_l(&self) -> &DmaCompParams2L {
        &self.dma_comp_params_2_l
    }
    #[doc = "0x3ec - "]
    #[inline(always)]
    pub const fn dma_comp_params_2_h(&self) -> &DmaCompParams2H {
        &self.dma_comp_params_2_h
    }
    #[doc = "0x3f0 - "]
    #[inline(always)]
    pub const fn dma_comp_params_1_l(&self) -> &DmaCompParams1L {
        &self.dma_comp_params_1_l
    }
    #[doc = "0x3f4 - "]
    #[inline(always)]
    pub const fn dma_comp_params_1_h(&self) -> &DmaCompParams1H {
        &self.dma_comp_params_1_h
    }
}
#[doc = "DMA Channel"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "DMA Channel"]
pub mod ch;
#[doc = "RAW_TFR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`raw_tfr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_tfr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_tfr_l`] module"]
#[doc(alias = "RAW_TFR_L")]
pub type RawTfrL = crate::Reg<raw_tfr_l::RawTfrLSpec>;
#[doc = ""]
pub mod raw_tfr_l;
#[doc = "RAW_TFR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`raw_tfr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_tfr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_tfr_h`] module"]
#[doc(alias = "RAW_TFR_H")]
pub type RawTfrH = crate::Reg<raw_tfr_h::RawTfrHSpec>;
#[doc = ""]
pub mod raw_tfr_h;
#[doc = "RAW_BLOCK_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`raw_block_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_block_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_block_l`] module"]
#[doc(alias = "RAW_BLOCK_L")]
pub type RawBlockL = crate::Reg<raw_block_l::RawBlockLSpec>;
#[doc = ""]
pub mod raw_block_l;
#[doc = "RAW_BLOCK_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`raw_block_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_block_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_block_h`] module"]
#[doc(alias = "RAW_BLOCK_H")]
pub type RawBlockH = crate::Reg<raw_block_h::RawBlockHSpec>;
#[doc = ""]
pub mod raw_block_h;
#[doc = "STATUS_TFR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status_tfr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_tfr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_tfr_l`] module"]
#[doc(alias = "STATUS_TFR_L")]
pub type StatusTfrL = crate::Reg<status_tfr_l::StatusTfrLSpec>;
#[doc = ""]
pub mod status_tfr_l;
#[doc = "STATUS_TFR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status_tfr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_tfr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_tfr_h`] module"]
#[doc(alias = "STATUS_TFR_H")]
pub type StatusTfrH = crate::Reg<status_tfr_h::StatusTfrHSpec>;
#[doc = ""]
pub mod status_tfr_h;
#[doc = "STATUS_BLOCK_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status_block_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_block_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_block_l`] module"]
#[doc(alias = "STATUS_BLOCK_L")]
pub type StatusBlockL = crate::Reg<status_block_l::StatusBlockLSpec>;
#[doc = ""]
pub mod status_block_l;
#[doc = "STATUS_BLOCK_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status_block_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_block_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_block_h`] module"]
#[doc(alias = "STATUS_BLOCK_H")]
pub type StatusBlockH = crate::Reg<status_block_h::StatusBlockHSpec>;
#[doc = ""]
pub mod status_block_h;
#[doc = "MASK_TFR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mask_tfr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tfr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_tfr_l`] module"]
#[doc(alias = "MASK_TFR_L")]
pub type MaskTfrL = crate::Reg<mask_tfr_l::MaskTfrLSpec>;
#[doc = ""]
pub mod mask_tfr_l;
#[doc = "MASK_TFR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mask_tfr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_tfr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_tfr_h`] module"]
#[doc(alias = "MASK_TFR_H")]
pub type MaskTfrH = crate::Reg<mask_tfr_h::MaskTfrHSpec>;
#[doc = ""]
pub mod mask_tfr_h;
#[doc = "MASK_BLOCK_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mask_block_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_block_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_block_l`] module"]
#[doc(alias = "MASK_BLOCK_L")]
pub type MaskBlockL = crate::Reg<mask_block_l::MaskBlockLSpec>;
#[doc = ""]
pub mod mask_block_l;
#[doc = "MASK_BLOCK_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mask_block_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_block_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_block_h`] module"]
#[doc(alias = "MASK_BLOCK_H")]
pub type MaskBlockH = crate::Reg<mask_block_h::MaskBlockHSpec>;
#[doc = ""]
pub mod mask_block_h;
#[doc = "CLEAR_TFR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_tfr_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_tfr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_tfr_l`] module"]
#[doc(alias = "CLEAR_TFR_L")]
pub type ClearTfrL = crate::Reg<clear_tfr_l::ClearTfrLSpec>;
#[doc = ""]
pub mod clear_tfr_l;
#[doc = "CLEAR_TFR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_tfr_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_tfr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_tfr_h`] module"]
#[doc(alias = "CLEAR_TFR_H")]
pub type ClearTfrH = crate::Reg<clear_tfr_h::ClearTfrHSpec>;
#[doc = ""]
pub mod clear_tfr_h;
#[doc = "CLEAR_BLOCK_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_block_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_block_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_block_l`] module"]
#[doc(alias = "CLEAR_BLOCK_L")]
pub type ClearBlockL = crate::Reg<clear_block_l::ClearBlockLSpec>;
#[doc = ""]
pub mod clear_block_l;
#[doc = "CLEAR_BLOCK_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_block_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_block_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_block_h`] module"]
#[doc(alias = "CLEAR_BLOCK_H")]
pub type ClearBlockH = crate::Reg<clear_block_h::ClearBlockHSpec>;
#[doc = ""]
pub mod clear_block_h;
#[doc = "CLEAR_SRC_TRAN_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_src_tran_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_src_tran_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_src_tran_l`] module"]
#[doc(alias = "CLEAR_SRC_TRAN_L")]
pub type ClearSrcTranL = crate::Reg<clear_src_tran_l::ClearSrcTranLSpec>;
#[doc = ""]
pub mod clear_src_tran_l;
#[doc = "CLEAR_SRC_TRAN_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_src_tran_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_src_tran_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_src_tran_h`] module"]
#[doc(alias = "CLEAR_SRC_TRAN_H")]
pub type ClearSrcTranH = crate::Reg<clear_src_tran_h::ClearSrcTranHSpec>;
#[doc = ""]
pub mod clear_src_tran_h;
#[doc = "CLEAR_DST_TRAN_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_dst_tran_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_dst_tran_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_dst_tran_l`] module"]
#[doc(alias = "CLEAR_DST_TRAN_L")]
pub type ClearDstTranL = crate::Reg<clear_dst_tran_l::ClearDstTranLSpec>;
#[doc = ""]
pub mod clear_dst_tran_l;
#[doc = "CLEAR_DST_TRAN_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_dst_tran_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_dst_tran_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_dst_tran_h`] module"]
#[doc(alias = "CLEAR_DST_TRAN_H")]
pub type ClearDstTranH = crate::Reg<clear_dst_tran_h::ClearDstTranHSpec>;
#[doc = ""]
pub mod clear_dst_tran_h;
#[doc = "CLEAR_ERR_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_err_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_err_l`] module"]
#[doc(alias = "CLEAR_ERR_L")]
pub type ClearErrL = crate::Reg<clear_err_l::ClearErrLSpec>;
#[doc = ""]
pub mod clear_err_l;
#[doc = "CLEAR_ERR_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clear_err_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_err_h`] module"]
#[doc(alias = "CLEAR_ERR_H")]
pub type ClearErrH = crate::Reg<clear_err_h::ClearErrHSpec>;
#[doc = ""]
pub mod clear_err_h;
#[doc = "STATUS_INT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status_int::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_int`] module"]
#[doc(alias = "STATUS_INT")]
pub type StatusInt = crate::Reg<status_int::StatusIntSpec>;
#[doc = ""]
pub mod status_int;
#[doc = "REQ_SRC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`req_src::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@req_src`] module"]
#[doc(alias = "REQ_SRC")]
pub type ReqSrc = crate::Reg<req_src::ReqSrcSpec>;
#[doc = ""]
pub mod req_src;
#[doc = "REQ_DST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`req_dst::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@req_dst`] module"]
#[doc(alias = "REQ_DST")]
pub type ReqDst = crate::Reg<req_dst::ReqDstSpec>;
#[doc = ""]
pub mod req_dst;
#[doc = "SGL_REQ_SRC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sgl_req_src::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgl_req_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgl_req_src`] module"]
#[doc(alias = "SGL_REQ_SRC")]
pub type SglReqSrc = crate::Reg<sgl_req_src::SglReqSrcSpec>;
#[doc = ""]
pub mod sgl_req_src;
#[doc = "SGL_REQ_DST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sgl_req_dst::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgl_req_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgl_req_dst`] module"]
#[doc(alias = "SGL_REQ_DST")]
pub type SglReqDst = crate::Reg<sgl_req_dst::SglReqDstSpec>;
#[doc = ""]
pub mod sgl_req_dst;
#[doc = "DMACFGREG_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgreg_l`] module"]
#[doc(alias = "DMACFGREG_L")]
pub type DmacfgregL = crate::Reg<dmacfgreg_l::DmacfgregLSpec>;
#[doc = ""]
pub mod dmacfgreg_l;
#[doc = "DMACFGREG_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgreg_h`] module"]
#[doc(alias = "DMACFGREG_H")]
pub type DmacfgregH = crate::Reg<dmacfgreg_h::DmacfgregHSpec>;
#[doc = ""]
pub mod dmacfgreg_h;
#[doc = "CHENREG_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenreg_l`] module"]
#[doc(alias = "CHENREG_L")]
pub type ChenregL = crate::Reg<chenreg_l::ChenregLSpec>;
#[doc = ""]
pub mod chenreg_l;
#[doc = "CHENREG_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenreg_h`] module"]
#[doc(alias = "CHENREG_H")]
pub type ChenregH = crate::Reg<chenreg_h::ChenregHSpec>;
#[doc = ""]
pub mod chenreg_h;
#[doc = "DMA_COMP_PARAMS_3_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_3_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_3_l`] module"]
#[doc(alias = "DMA_COMP_PARAMS_3_L")]
pub type DmaCompParams3L = crate::Reg<dma_comp_params_3_l::DmaCompParams3LSpec>;
#[doc = ""]
pub mod dma_comp_params_3_l;
#[doc = "DMA_COMP_PARAMS_3_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_3_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_3_h`] module"]
#[doc(alias = "DMA_COMP_PARAMS_3_H")]
pub type DmaCompParams3H = crate::Reg<dma_comp_params_3_h::DmaCompParams3HSpec>;
#[doc = ""]
pub mod dma_comp_params_3_h;
#[doc = "DMA_COMP_PARAMS_2_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_2_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_2_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_2_l`] module"]
#[doc(alias = "DMA_COMP_PARAMS_2_L")]
pub type DmaCompParams2L = crate::Reg<dma_comp_params_2_l::DmaCompParams2LSpec>;
#[doc = ""]
pub mod dma_comp_params_2_l;
#[doc = "DMA_COMP_PARAMS_2_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_2_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_2_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_2_h`] module"]
#[doc(alias = "DMA_COMP_PARAMS_2_H")]
pub type DmaCompParams2H = crate::Reg<dma_comp_params_2_h::DmaCompParams2HSpec>;
#[doc = ""]
pub mod dma_comp_params_2_h;
#[doc = "DMA_COMP_PARAMS_1_L (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_1_l::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_1_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_1_l`] module"]
#[doc(alias = "DMA_COMP_PARAMS_1_L")]
pub type DmaCompParams1L = crate::Reg<dma_comp_params_1_l::DmaCompParams1LSpec>;
#[doc = ""]
pub mod dma_comp_params_1_l;
#[doc = "DMA_COMP_PARAMS_1_H (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_1_h::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_comp_params_1_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_comp_params_1_h`] module"]
#[doc(alias = "DMA_COMP_PARAMS_1_H")]
pub type DmaCompParams1H = crate::Reg<dma_comp_params_1_h::DmaCompParams1HSpec>;
#[doc = ""]
pub mod dma_comp_params_1_h;
