#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `DBL_POSITION` reader - Dbl position"]
pub type DblPositionR = crate::BitReader;
#[doc = "Field `DBL_POSITION` writer - Dbl position"]
pub type DblPositionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Dbl position"]
    #[inline(always)]
    pub fn dbl_position(&self) -> DblPositionR {
        DblPositionR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Dbl position"]
    #[inline(always)]
    pub fn dbl_position(&mut self) -> DblPositionW<'_, DcrSpec> {
        DblPositionW::new(self, 3)
    }
}
#[doc = "TIMER DMA control register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
