#[doc = "Register `SAECR` reader"]
pub type R = crate::R<SaecrSpec>;
#[doc = "Register `SAECR` writer"]
pub type W = crate::W<SaecrSpec>;
#[doc = "Field `WORD_WRITE` reader - Word write"]
pub type WordWriteR = crate::BitReader;
#[doc = "Field `WORD_WRITE` writer - Word write"]
pub type WordWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIAN_TRANSFORM` reader - Endian transform"]
pub type EndianTransformR = crate::BitReader;
#[doc = "Field `ENDIAN_TRANSFORM` writer - Endian transform"]
pub type EndianTransformW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE_ENABLE` reader - Dma mode enable"]
pub type DmaModeEnableR = crate::BitReader;
#[doc = "Field `DMA_MODE_ENABLE` writer - Dma mode enable"]
pub type DmaModeEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE` reader - Interrupt enable"]
pub type InterruptEnableR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE` writer - Interrupt enable"]
pub type InterruptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Word write"]
    #[inline(always)]
    pub fn word_write(&self) -> WordWriteR {
        WordWriteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endian transform"]
    #[inline(always)]
    pub fn endian_transform(&self) -> EndianTransformR {
        EndianTransformR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dma mode enable"]
    #[inline(always)]
    pub fn dma_mode_enable(&self) -> DmaModeEnableR {
        DmaModeEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> InterruptEnableR {
        InterruptEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Word write"]
    #[inline(always)]
    pub fn word_write(&mut self) -> WordWriteW<'_, SaecrSpec> {
        WordWriteW::new(self, 4)
    }
    #[doc = "Bit 5 - Endian transform"]
    #[inline(always)]
    pub fn endian_transform(&mut self) -> EndianTransformW<'_, SaecrSpec> {
        EndianTransformW::new(self, 5)
    }
    #[doc = "Bit 6 - Dma mode enable"]
    #[inline(always)]
    pub fn dma_mode_enable(&mut self) -> DmaModeEnableW<'_, SaecrSpec> {
        DmaModeEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn interrupt_enable(&mut self) -> InterruptEnableW<'_, SaecrSpec> {
        InterruptEnableW::new(self, 7)
    }
}
#[doc = "Common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saecr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaecrSpec;
impl crate::RegisterSpec for SaecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saecr::R`](R) reader structure"]
impl crate::Readable for SaecrSpec {}
#[doc = "`write(|w| ..)` method takes [`saecr::W`](W) writer structure"]
impl crate::Writable for SaecrSpec {
    type Safety = crate::Unsafe;
}
