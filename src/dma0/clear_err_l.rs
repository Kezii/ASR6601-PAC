#[doc = "Register `CLEAR_ERR_L` reader"]
pub type R = crate::R<ClearErrLSpec>;
#[doc = "Register `CLEAR_ERR_L` writer"]
pub type W = crate::W<ClearErrLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_err_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearErrLSpec;
impl crate::RegisterSpec for ClearErrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_err_l::R`](R) reader structure"]
impl crate::Readable for ClearErrLSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_err_l::W`](W) writer structure"]
impl crate::Writable for ClearErrLSpec {
    type Safety = crate::Unsafe;
}
