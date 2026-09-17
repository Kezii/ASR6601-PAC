#[doc = "Register `MASK_SRC_TRAN` reader"]
pub type R = crate::R<MaskSrcTranSpec>;
#[doc = "Register `MASK_SRC_TRAN` writer"]
pub type W = crate::W<MaskSrcTranSpec>;
#[doc = "Field `INT_MASK_0` reader - DMA channel 0 source processing completed interrupt enable"]
pub type IntMask0R = crate::BitReader;
#[doc = "Field `INT_MASK_0` writer - DMA channel 0 source processing completed interrupt enable"]
pub type IntMask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_1` reader - DMA channel 1 source processing completed interrupt enable"]
pub type IntMask1R = crate::BitReader;
#[doc = "Field `INT_MASK_1` writer - DMA channel 1 source processing completed interrupt enable"]
pub type IntMask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_2` reader - DMA channel 2 source processing completed interrupt enable"]
pub type IntMask2R = crate::BitReader;
#[doc = "Field `INT_MASK_2` writer - DMA channel 2 source processing completed interrupt enable"]
pub type IntMask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_3` reader - DMA channel 3 source processing completed interrupt enable"]
pub type IntMask3R = crate::BitReader;
#[doc = "Field `INT_MASK_3` writer - DMA channel 3 source processing completed interrupt enable"]
pub type IntMask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_WE_0` reader - DMA channel 0 source processing completed interrupt mask write enable"]
pub type IntMaskWe0R = crate::BitReader;
#[doc = "Field `INT_MASK_WE_0` writer - DMA channel 0 source processing completed interrupt mask write enable"]
pub type IntMaskWe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_WE_1` reader - DMA channel 1 source processing completed interrupt mask write enable"]
pub type IntMaskWe1R = crate::BitReader;
#[doc = "Field `INT_MASK_WE_1` writer - DMA channel 1 source processing completed interrupt mask write enable"]
pub type IntMaskWe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_WE_2` reader - DMA channel 2 source processing completed interrupt mask write enable"]
pub type IntMaskWe2R = crate::BitReader;
#[doc = "Field `INT_MASK_WE_2` writer - DMA channel 2 source processing completed interrupt mask write enable"]
pub type IntMaskWe2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_MASK_WE_3` reader - DMA channel 3 source processing completed interrupt mask write enable"]
pub type IntMaskWe3R = crate::BitReader;
#[doc = "Field `INT_MASK_WE_3` writer - DMA channel 3 source processing completed interrupt mask write enable"]
pub type IntMaskWe3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA channel 0 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_0(&self) -> IntMask0R {
        IntMask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA channel 1 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_1(&self) -> IntMask1R {
        IntMask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA channel 2 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_2(&self) -> IntMask2R {
        IntMask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA channel 3 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_3(&self) -> IntMask3R {
        IntMask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA channel 0 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_0(&self) -> IntMaskWe0R {
        IntMaskWe0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA channel 1 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_1(&self) -> IntMaskWe1R {
        IntMaskWe1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA channel 2 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_2(&self) -> IntMaskWe2R {
        IntMaskWe2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA channel 3 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_3(&self) -> IntMaskWe3R {
        IntMaskWe3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA channel 0 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_0(&mut self) -> IntMask0W<'_, MaskSrcTranSpec> {
        IntMask0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA channel 1 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_1(&mut self) -> IntMask1W<'_, MaskSrcTranSpec> {
        IntMask1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA channel 2 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_2(&mut self) -> IntMask2W<'_, MaskSrcTranSpec> {
        IntMask2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA channel 3 source processing completed interrupt enable"]
    #[inline(always)]
    pub fn int_mask_3(&mut self) -> IntMask3W<'_, MaskSrcTranSpec> {
        IntMask3W::new(self, 3)
    }
    #[doc = "Bit 8 - DMA channel 0 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_0(&mut self) -> IntMaskWe0W<'_, MaskSrcTranSpec> {
        IntMaskWe0W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA channel 1 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_1(&mut self) -> IntMaskWe1W<'_, MaskSrcTranSpec> {
        IntMaskWe1W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA channel 2 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_2(&mut self) -> IntMaskWe2W<'_, MaskSrcTranSpec> {
        IntMaskWe2W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA channel 3 source processing completed interrupt mask write enable"]
    #[inline(always)]
    pub fn int_mask_we_3(&mut self) -> IntMaskWe3W<'_, MaskSrcTranSpec> {
        IntMaskWe3W::new(self, 11)
    }
}
#[doc = "DMA source processing completed interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_src_tran::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_src_tran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSrcTranSpec;
impl crate::RegisterSpec for MaskSrcTranSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mask_src_tran::R`](R) reader structure"]
impl crate::Readable for MaskSrcTranSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_src_tran::W`](W) writer structure"]
impl crate::Writable for MaskSrcTranSpec {
    type Safety = crate::Unsafe;
}
