#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `CH_PRIOR` reader - DMA channel priority configuration"]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `CH_PRIOR` writer - DMA channel priority configuration"]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH_SUSP` reader - DMA channel FIFO suspend indication"]
pub type ChSuspR = crate::BitReader;
#[doc = "Field `CH_SUSP` writer - DMA channel FIFO suspend indication"]
pub type ChSuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY` reader - DMA channel FIFO empty indication"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` writer - DMA channel FIFO empty indication"]
pub type FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_SEL_DST` reader - DMA destination handshake signal selection"]
pub type HsSelDstR = crate::BitReader;
#[doc = "Field `HS_SEL_DST` writer - DMA destination handshake signal selection"]
pub type HsSelDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_SEL_SRC` reader - DMA source handshake signal selection"]
pub type HsSelSrcR = crate::BitReader;
#[doc = "Field `HS_SEL_SRC` writer - DMA source handshake signal selection"]
pub type HsSelSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA channel lock delay"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockChL {
    #[doc = "0: wait until the DMA transfer is completed"]
    TransferCompleted = 0,
    #[doc = "1: wait until the block transfer is completed"]
    BlockCompleted = 1,
    #[doc = "2: wait until DMA processing is completed"]
    DmaProcessingCompleted = 2,
}
impl From<LockChL> for u8 {
    #[inline(always)]
    fn from(variant: LockChL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockChL {
    type Ux = u8;
}
impl crate::IsEnum for LockChL {}
#[doc = "Field `LOCK_CH_L` reader - DMA channel lock delay"]
pub type LockChLR = crate::FieldReader<LockChL>;
impl LockChLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockChL> {
        match self.bits {
            0 => Some(LockChL::TransferCompleted),
            1 => Some(LockChL::BlockCompleted),
            2 => Some(LockChL::DmaProcessingCompleted),
            _ => None,
        }
    }
    #[doc = "wait until the DMA transfer is completed"]
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == LockChL::TransferCompleted
    }
    #[doc = "wait until the block transfer is completed"]
    #[inline(always)]
    pub fn is_block_completed(&self) -> bool {
        *self == LockChL::BlockCompleted
    }
    #[doc = "wait until DMA processing is completed"]
    #[inline(always)]
    pub fn is_dma_processing_completed(&self) -> bool {
        *self == LockChL::DmaProcessingCompleted
    }
}
#[doc = "Field `LOCK_CH_L` writer - DMA channel lock delay"]
pub type LockChLW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockChL>;
impl<'a, REG> LockChLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wait until the DMA transfer is completed"]
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::TransferCompleted)
    }
    #[doc = "wait until the block transfer is completed"]
    #[inline(always)]
    pub fn block_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::BlockCompleted)
    }
    #[doc = "wait until DMA processing is completed"]
    #[inline(always)]
    pub fn dma_processing_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockChL::DmaProcessingCompleted)
    }
}
#[doc = "Bus lock delay"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockBL {
    #[doc = "0: wait until the DMA transfer is completed"]
    TransferCompleted = 0,
    #[doc = "1: wait until the block transfer is completed"]
    BlockCompleted = 1,
    #[doc = "2: wait until DMA processing is completed"]
    DmaProcessingCompleted = 2,
}
impl From<LockBL> for u8 {
    #[inline(always)]
    fn from(variant: LockBL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockBL {
    type Ux = u8;
}
impl crate::IsEnum for LockBL {}
#[doc = "Field `LOCK_B_L` reader - Bus lock delay"]
pub type LockBLR = crate::FieldReader<LockBL>;
impl LockBLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockBL> {
        match self.bits {
            0 => Some(LockBL::TransferCompleted),
            1 => Some(LockBL::BlockCompleted),
            2 => Some(LockBL::DmaProcessingCompleted),
            _ => None,
        }
    }
    #[doc = "wait until the DMA transfer is completed"]
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == LockBL::TransferCompleted
    }
    #[doc = "wait until the block transfer is completed"]
    #[inline(always)]
    pub fn is_block_completed(&self) -> bool {
        *self == LockBL::BlockCompleted
    }
    #[doc = "wait until DMA processing is completed"]
    #[inline(always)]
    pub fn is_dma_processing_completed(&self) -> bool {
        *self == LockBL::DmaProcessingCompleted
    }
}
#[doc = "Field `LOCK_B_L` writer - Bus lock delay"]
pub type LockBLW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockBL>;
impl<'a, REG> LockBLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wait until the DMA transfer is completed"]
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::TransferCompleted)
    }
    #[doc = "wait until the block transfer is completed"]
    #[inline(always)]
    pub fn block_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::BlockCompleted)
    }
    #[doc = "wait until DMA processing is completed"]
    #[inline(always)]
    pub fn dma_processing_completed(self) -> &'a mut crate::W<REG> {
        self.variant(LockBL::DmaProcessingCompleted)
    }
}
#[doc = "Field `LOCK_CH` reader - DMA channel lock control"]
pub type LockChR = crate::BitReader;
#[doc = "Field `LOCK_CH` writer - DMA channel lock control"]
pub type LockChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK_B` reader - Bus lock control"]
pub type LockBR = crate::BitReader;
#[doc = "Field `LOCK_B` writer - Bus lock control"]
pub type LockBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_HS_POL` reader - DMA destination handshake information polarity"]
pub type DstHsPolR = crate::BitReader;
#[doc = "Field `DST_HS_POL` writer - DMA destination handshake information polarity"]
pub type DstHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_HS_POL` reader - DMA source handshake information polarity"]
pub type SrcHsPolR = crate::BitReader;
#[doc = "Field `SRC_HS_POL` writer - DMA source handshake information polarity"]
pub type SrcHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELOAD_SRC` reader - DMA source auto-reloading enable"]
pub type ReloadSrcR = crate::BitReader;
#[doc = "Field `RELOAD_SRC` writer - DMA source auto-reloading enable"]
pub type ReloadSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELOAD_DST` reader - DMA destination auto-reloading enable"]
pub type ReloadDstR = crate::BitReader;
#[doc = "Field `RELOAD_DST` writer - DMA destination auto-reloading enable"]
pub type ReloadDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMODE` reader - Source flow control mode selection"]
pub type FcmodeR = crate::BitReader;
#[doc = "Field `FCMODE` writer - Source flow control mode selection"]
pub type FcmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_MODE` reader - FIFO mode selection"]
pub type FifoModeR = crate::BitReader;
#[doc = "Field `FIFO_MODE` writer - FIFO mode selection"]
pub type FifoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTCTL` reader - Protection control"]
pub type ProtctlR = crate::FieldReader;
#[doc = "Field `PROTCTL` writer - Protection control"]
pub type ProtctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS_UPD_EN` reader - DMA destination status update enable"]
pub type DsUpdEnR = crate::BitReader;
#[doc = "Field `DS_UPD_EN` writer - DMA destination status update enable"]
pub type DsUpdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS_UPD_EN` reader - DMA source status update enable"]
pub type SsUpdEnR = crate::BitReader;
#[doc = "Field `SS_UPD_EN` writer - DMA source status update enable"]
pub type SsUpdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_PER` reader - DMA source handshake interface"]
pub type SrcPerR = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - DMA source handshake interface"]
pub type SrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEST_PER` reader - DMA destination handshake interface"]
pub type DestPerR = crate::FieldReader;
#[doc = "Field `DEST_PER` writer - DMA destination handshake interface"]
pub type DestPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 5:7 - DMA channel priority configuration"]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DMA channel FIFO suspend indication"]
    #[inline(always)]
    pub fn ch_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA channel FIFO empty indication"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA destination handshake signal selection"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HsSelDstR {
        HsSelDstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA source handshake signal selection"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HsSelSrcR {
        HsSelSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DMA channel lock delay"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LockChLR {
        LockChLR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Bus lock delay"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LockBLR {
        LockBLR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - DMA channel lock control"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LockChR {
        LockChR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus lock control"]
    #[inline(always)]
    pub fn lock_b(&self) -> LockBR {
        LockBR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA destination handshake information polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DstHsPolR {
        DstHsPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA source handshake information polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SrcHsPolR {
        SrcHsPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA source auto-reloading enable"]
    #[inline(always)]
    pub fn reload_src(&self) -> ReloadSrcR {
        ReloadSrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA destination auto-reloading enable"]
    #[inline(always)]
    pub fn reload_dst(&self) -> ReloadDstR {
        ReloadDstR::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 32 - Source flow control mode selection"]
    #[inline(always)]
    pub fn fcmode(&self) -> FcmodeR {
        FcmodeR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - FIFO mode selection"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bits 34:36 - Protection control"]
    #[inline(always)]
    pub fn protctl(&self) -> ProtctlR {
        ProtctlR::new(((self.bits >> 34) & 7) as u8)
    }
    #[doc = "Bit 37 - DMA destination status update enable"]
    #[inline(always)]
    pub fn ds_upd_en(&self) -> DsUpdEnR {
        DsUpdEnR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - DMA source status update enable"]
    #[inline(always)]
    pub fn ss_upd_en(&self) -> SsUpdEnR {
        SsUpdEnR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 39:42 - DMA source handshake interface"]
    #[inline(always)]
    pub fn src_per(&self) -> SrcPerR {
        SrcPerR::new(((self.bits >> 39) & 0x0f) as u8)
    }
    #[doc = "Bits 43:46 - DMA destination handshake interface"]
    #[inline(always)]
    pub fn dest_per(&self) -> DestPerR {
        DestPerR::new(((self.bits >> 43) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7 - DMA channel priority configuration"]
    #[inline(always)]
    pub fn ch_prior(&mut self) -> ChPriorW<'_, Cfg0Spec> {
        ChPriorW::new(self, 5)
    }
    #[doc = "Bit 8 - DMA channel FIFO suspend indication"]
    #[inline(always)]
    pub fn ch_susp(&mut self) -> ChSuspW<'_, Cfg0Spec> {
        ChSuspW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA channel FIFO empty indication"]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> FifoEmptyW<'_, Cfg0Spec> {
        FifoEmptyW::new(self, 9)
    }
    #[doc = "Bit 10 - DMA destination handshake signal selection"]
    #[inline(always)]
    pub fn hs_sel_dst(&mut self) -> HsSelDstW<'_, Cfg0Spec> {
        HsSelDstW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA source handshake signal selection"]
    #[inline(always)]
    pub fn hs_sel_src(&mut self) -> HsSelSrcW<'_, Cfg0Spec> {
        HsSelSrcW::new(self, 11)
    }
    #[doc = "Bits 12:13 - DMA channel lock delay"]
    #[inline(always)]
    pub fn lock_ch_l(&mut self) -> LockChLW<'_, Cfg0Spec> {
        LockChLW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Bus lock delay"]
    #[inline(always)]
    pub fn lock_b_l(&mut self) -> LockBLW<'_, Cfg0Spec> {
        LockBLW::new(self, 14)
    }
    #[doc = "Bit 16 - DMA channel lock control"]
    #[inline(always)]
    pub fn lock_ch(&mut self) -> LockChW<'_, Cfg0Spec> {
        LockChW::new(self, 16)
    }
    #[doc = "Bit 17 - Bus lock control"]
    #[inline(always)]
    pub fn lock_b(&mut self) -> LockBW<'_, Cfg0Spec> {
        LockBW::new(self, 17)
    }
    #[doc = "Bit 18 - DMA destination handshake information polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&mut self) -> DstHsPolW<'_, Cfg0Spec> {
        DstHsPolW::new(self, 18)
    }
    #[doc = "Bit 19 - DMA source handshake information polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&mut self) -> SrcHsPolW<'_, Cfg0Spec> {
        SrcHsPolW::new(self, 19)
    }
    #[doc = "Bit 30 - DMA source auto-reloading enable"]
    #[inline(always)]
    pub fn reload_src(&mut self) -> ReloadSrcW<'_, Cfg0Spec> {
        ReloadSrcW::new(self, 30)
    }
    #[doc = "Bit 31 - DMA destination auto-reloading enable"]
    #[inline(always)]
    pub fn reload_dst(&mut self) -> ReloadDstW<'_, Cfg0Spec> {
        ReloadDstW::new(self, 31)
    }
    #[doc = "Bit 32 - Source flow control mode selection"]
    #[inline(always)]
    pub fn fcmode(&mut self) -> FcmodeW<'_, Cfg0Spec> {
        FcmodeW::new(self, 32)
    }
    #[doc = "Bit 33 - FIFO mode selection"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FifoModeW<'_, Cfg0Spec> {
        FifoModeW::new(self, 33)
    }
    #[doc = "Bits 34:36 - Protection control"]
    #[inline(always)]
    pub fn protctl(&mut self) -> ProtctlW<'_, Cfg0Spec> {
        ProtctlW::new(self, 34)
    }
    #[doc = "Bit 37 - DMA destination status update enable"]
    #[inline(always)]
    pub fn ds_upd_en(&mut self) -> DsUpdEnW<'_, Cfg0Spec> {
        DsUpdEnW::new(self, 37)
    }
    #[doc = "Bit 38 - DMA source status update enable"]
    #[inline(always)]
    pub fn ss_upd_en(&mut self) -> SsUpdEnW<'_, Cfg0Spec> {
        SsUpdEnW::new(self, 38)
    }
    #[doc = "Bits 39:42 - DMA source handshake interface"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SrcPerW<'_, Cfg0Spec> {
        SrcPerW::new(self, 39)
    }
    #[doc = "Bits 43:46 - DMA destination handshake interface"]
    #[inline(always)]
    pub fn dest_per(&mut self) -> DestPerW<'_, Cfg0Spec> {
        DestPerW::new(self, 43)
    }
}
#[doc = "channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
