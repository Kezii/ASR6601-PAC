#[doc = "Register `CR9` reader"]
pub type R = crate::R<Cr9Spec>;
#[doc = "Register `CR9` writer"]
pub type W = crate::W<Cr9Spec>;
#[doc = "Field `QSPI_REMAP_DST_ADDR` reader - qspi remap destination address, aligned in 1kb"]
pub type QspiRemapDstAddrR = crate::FieldReader<u16>;
#[doc = "Field `QSPI_REMAP_DST_ADDR` writer - qspi remap destination address, aligned in 1kb"]
pub type QspiRemapDstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `QSPI_REMAP_SRC_ADDR` reader - qspi remap source address, aligned in 1kb"]
pub type QspiRemapSrcAddrR = crate::FieldReader<u16>;
#[doc = "Field `QSPI_REMAP_SRC_ADDR` writer - qspi remap source address, aligned in 1kb"]
pub type QspiRemapSrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - qspi remap destination address, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_dst_addr(&self) -> QspiRemapDstAddrR {
        QspiRemapDstAddrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - qspi remap source address, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_src_addr(&self) -> QspiRemapSrcAddrR {
        QspiRemapSrcAddrR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - qspi remap destination address, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_dst_addr(&mut self) -> QspiRemapDstAddrW<'_, Cr9Spec> {
        QspiRemapDstAddrW::new(self, 0)
    }
    #[doc = "Bits 14:27 - qspi remap source address, aligned in 1kb"]
    #[inline(always)]
    pub fn qspi_remap_src_addr(&mut self) -> QspiRemapSrcAddrW<'_, Cr9Spec> {
        QspiRemapSrcAddrW::new(self, 14)
    }
}
#[doc = "control register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`cr9::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr9Spec;
impl crate::RegisterSpec for Cr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr9::R`](R) reader structure"]
impl crate::Readable for Cr9Spec {}
#[doc = "`write(|w| ..)` method takes [`cr9::W`](W) writer structure"]
impl crate::Writable for Cr9Spec {
    type Safety = crate::Unsafe;
}
