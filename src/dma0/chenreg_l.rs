#[doc = "Register `CHENREG_L` reader"]
pub type R = crate::R<ChenregLSpec>;
#[doc = "Register `CHENREG_L` writer"]
pub type W = crate::W<ChenregLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenregLSpec;
impl crate::RegisterSpec for ChenregLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenreg_l::R`](R) reader structure"]
impl crate::Readable for ChenregLSpec {}
#[doc = "`write(|w| ..)` method takes [`chenreg_l::W`](W) writer structure"]
impl crate::Writable for ChenregLSpec {
    type Safety = crate::Unsafe;
}
