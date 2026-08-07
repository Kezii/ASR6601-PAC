#[doc = "Register `CLEAR_ERR_H` reader"]
pub type R = crate::R<ClearErrHSpec>;
#[doc = "Register `CLEAR_ERR_H` writer"]
pub type W = crate::W<ClearErrHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_err_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_err_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearErrHSpec;
impl crate::RegisterSpec for ClearErrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_err_h::R`](R) reader structure"]
impl crate::Readable for ClearErrHSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_err_h::W`](W) writer structure"]
impl crate::Writable for ClearErrHSpec {
    type Safety = crate::Unsafe;
}
