#[doc = "Register `WCR` reader"]
pub type R = crate::R<WcrSpec>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
#[doc = "Field `COUNT` reader - counter values for setup and hold times in standard and fast modes"]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - counter values for setup and hold times in standard and fast modes"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - counter values for setup and hold times in standard and fast modes"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - counter values for setup and hold times in standard and fast modes"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, WcrSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "wait count register\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
}
