#[doc = "Register `RST_CYCL` reader"]
pub type R = crate::R<RstCyclSpec>;
#[doc = "Register `RST_CYCL` writer"]
pub type W = crate::W<RstCyclSpec>;
#[doc = "Field `RST_CYC` reader - serial bus reset scl cycle count"]
pub type RstCycR = crate::FieldReader;
#[doc = "Field `RST_CYC` writer - serial bus reset scl cycle count"]
pub type RstCycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - serial bus reset scl cycle count"]
    #[inline(always)]
    pub fn rst_cyc(&self) -> RstCycR {
        RstCycR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - serial bus reset scl cycle count"]
    #[inline(always)]
    pub fn rst_cyc(&mut self) -> RstCycW<'_, RstCyclSpec> {
        RstCycW::new(self, 0)
    }
}
#[doc = "reset cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cycl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cycl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCyclSpec;
impl crate::RegisterSpec for RstCyclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_cycl::R`](R) reader structure"]
impl crate::Readable for RstCyclSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cycl::W`](W) writer structure"]
impl crate::Writable for RstCyclSpec {
    type Safety = crate::Unsafe;
}
