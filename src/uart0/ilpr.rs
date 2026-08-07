#[doc = "Register `ILPR` reader"]
pub type R = crate::R<IlprSpec>;
#[doc = "Register `ILPR` writer"]
pub type W = crate::W<IlprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IRDA low power counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilpr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlprSpec;
impl crate::RegisterSpec for IlprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilpr::R`](R) reader structure"]
impl crate::Readable for IlprSpec {}
#[doc = "`write(|w| ..)` method takes [`ilpr::W`](W) writer structure"]
impl crate::Writable for IlprSpec {
    type Safety = crate::Unsafe;
}
