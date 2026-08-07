#[doc = "Register `CHENREG_H` reader"]
pub type R = crate::R<ChenregHSpec>;
#[doc = "Register `CHENREG_H` writer"]
pub type W = crate::W<ChenregHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenregHSpec;
impl crate::RegisterSpec for ChenregHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenreg_h::R`](R) reader structure"]
impl crate::Readable for ChenregHSpec {}
#[doc = "`write(|w| ..)` method takes [`chenreg_h::W`](W) writer structure"]
impl crate::Writable for ChenregHSpec {
    type Safety = crate::Unsafe;
}
