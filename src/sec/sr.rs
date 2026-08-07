#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `FLASH_ACCESS_ERROR` reader - Flash access error"]
pub type FlashAccessErrorR = crate::BitReader;
#[doc = "Field `FLASH_ACCESS_ERROR` writer - Flash access error"]
pub type FlashAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Flash access error"]
    #[inline(always)]
    pub fn flash_access_error(&self) -> FlashAccessErrorR {
        FlashAccessErrorR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Flash access error"]
    #[inline(always)]
    pub fn flash_access_error(&mut self) -> FlashAccessErrorW<'_, SrSpec> {
        FlashAccessErrorW::new(self, 12)
    }
}
#[doc = "status\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
