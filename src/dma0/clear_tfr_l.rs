#[doc = "Register `CLEAR_TFR_L` reader"]
pub type R = crate::R<ClearTfrLSpec>;
#[doc = "Register `CLEAR_TFR_L` writer"]
pub type W = crate::W<ClearTfrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_tfr_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_tfr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearTfrLSpec;
impl crate::RegisterSpec for ClearTfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_tfr_l::R`](R) reader structure"]
impl crate::Readable for ClearTfrLSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_tfr_l::W`](W) writer structure"]
impl crate::Writable for ClearTfrLSpec {
    type Safety = crate::Unsafe;
}
