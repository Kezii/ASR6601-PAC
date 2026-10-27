#[doc = "Register `MAX` reader"]
pub type R = crate::R<MaxSpec>;
#[doc = "Register `MAX` writer"]
pub type W = crate::W<MaxSpec>;
#[doc = "Field `MAX` reader - Reload value, 12-bit (vendor SDK clamps to 0x0FFF)"]
pub type MaxR = crate::FieldReader<u16>;
#[doc = "Field `MAX` writer - Reload value, 12-bit (vendor SDK clamps to 0x0FFF)"]
pub type MaxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Reload value, 12-bit (vendor SDK clamps to 0x0FFF)"]
    #[inline(always)]
    pub fn max(&self) -> MaxR {
        MaxR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Reload value, 12-bit (vendor SDK clamps to 0x0FFF)"]
    #[inline(always)]
    pub fn max(&mut self) -> MaxW<'_, MaxSpec> {
        MaxW::new(self, 0)
    }
}
#[doc = "max value register\n\nYou can [`read`](crate::Reg::read) this register and get [`max::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxSpec;
impl crate::RegisterSpec for MaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max::R`](R) reader structure"]
impl crate::Readable for MaxSpec {}
#[doc = "`write(|w| ..)` method takes [`max::W`](W) writer structure"]
impl crate::Writable for MaxSpec {
    type Safety = crate::Unsafe;
}
