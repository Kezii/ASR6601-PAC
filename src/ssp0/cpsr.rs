#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSDVSR` reader - clock prescale divisor, even number 2 to 254"]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - clock prescale divisor, even number 2 to 254"]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - clock prescale divisor, even number 2 to 254"]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock prescale divisor, even number 2 to 254"]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<'_, CpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
}
