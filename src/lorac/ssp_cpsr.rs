#[doc = "Register `SSP_CPSR` reader"]
pub type R = crate::R<SspCpsrSpec>;
#[doc = "Register `SSP_CPSR` writer"]
pub type W = crate::W<SspCpsrSpec>;
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
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<'_, SspCpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "ssp clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_cpsr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_cpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCpsrSpec;
impl crate::RegisterSpec for SspCpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_cpsr::R`](R) reader structure"]
impl crate::Readable for SspCpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cpsr::W`](W) writer structure"]
impl crate::Writable for SspCpsrSpec {
    type Safety = crate::Unsafe;
}
