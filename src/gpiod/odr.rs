#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `OD` reader - pin\\[15:0\\] output"]
pub type OdR = crate::FieldReader<u16>;
#[doc = "Field `OD` writer - pin\\[15:0\\] output"]
pub type OdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output"]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] output"]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, OdrSpec> {
        OdW::new(self, 0)
    }
}
#[doc = "output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrSpec;
impl crate::RegisterSpec for OdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for OdrSpec {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for OdrSpec {
    type Safety = crate::Unsafe;
}
