#[doc = "Register `DHR` reader"]
pub type R = crate::R<DhrSpec>;
#[doc = "Register `DHR` writer"]
pub type W = crate::W<DhrSpec>;
#[doc = "Field `DATA` reader - 12-bit conversion data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - 12-bit conversion data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 12-bit conversion data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 12-bit conversion data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DhrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhrSpec;
impl crate::RegisterSpec for DhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr::R`](R) reader structure"]
impl crate::Readable for DhrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr::W`](W) writer structure"]
impl crate::Writable for DhrSpec {
    type Safety = crate::Unsafe;
}
