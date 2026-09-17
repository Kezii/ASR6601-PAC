#[doc = "Register `CYC_MAX_VALUE` reader"]
pub type R = crate::R<CycMaxValueSpec>;
#[doc = "Register `CYC_MAX_VALUE` writer"]
pub type W = crate::W<CycMaxValueSpec>;
#[doc = "Field `CYC_MAX_VALUE` reader - Cyc max value"]
pub type CycMaxValueR = crate::FieldReader<u32>;
#[doc = "Field `CYC_MAX_VALUE` writer - Cyc max value"]
pub type CycMaxValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cyc max value"]
    #[inline(always)]
    pub fn cyc_max_value(&self) -> CycMaxValueR {
        CycMaxValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cyc max value"]
    #[inline(always)]
    pub fn cyc_max_value(&mut self) -> CycMaxValueW<'_, CycMaxValueSpec> {
        CycMaxValueW::new(self, 0)
    }
}
#[doc = "cyc max value\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_max_value::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cyc_max_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycMaxValueSpec;
impl crate::RegisterSpec for CycMaxValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cyc_max_value::R`](R) reader structure"]
impl crate::Readable for CycMaxValueSpec {}
#[doc = "`write(|w| ..)` method takes [`cyc_max_value::W`](W) writer structure"]
impl crate::Writable for CycMaxValueSpec {
    type Safety = crate::Unsafe;
}
