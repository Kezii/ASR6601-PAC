#[doc = "Register `WIN` reader"]
pub type R = crate::R<WinSpec>;
#[doc = "Register `WIN` writer"]
pub type W = crate::W<WinSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "window value register\n\nYou can [`read`](crate::Reg::read) this register and get [`win::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinSpec;
impl crate::RegisterSpec for WinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WinSpec {}
#[doc = "`write(|w| ..)` method takes [`win::W`](W) writer structure"]
impl crate::Writable for WinSpec {
    type Safety = crate::Unsafe;
}
