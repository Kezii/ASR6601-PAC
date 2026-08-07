#[doc = "Register `SSTATAR_H` reader"]
pub type R = crate::R<SstatarHSpec>;
#[doc = "Register `SSTATAR_H` writer"]
pub type W = crate::W<SstatarHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatarHSpec;
impl crate::RegisterSpec for SstatarHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar_h::R`](R) reader structure"]
impl crate::Readable for SstatarHSpec {}
#[doc = "`write(|w| ..)` method takes [`sstatar_h::W`](W) writer structure"]
impl crate::Writable for SstatarHSpec {
    type Safety = crate::Unsafe;
}
