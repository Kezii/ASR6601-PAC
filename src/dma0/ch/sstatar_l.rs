#[doc = "Register `SSTATAR_L` reader"]
pub type R = crate::R<SstatarLSpec>;
#[doc = "Register `SSTATAR_L` writer"]
pub type W = crate::W<SstatarLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sstatar_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstatar_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatarLSpec;
impl crate::RegisterSpec for SstatarLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstatar_l::R`](R) reader structure"]
impl crate::Readable for SstatarLSpec {}
#[doc = "`write(|w| ..)` method takes [`sstatar_l::W`](W) writer structure"]
impl crate::Writable for SstatarLSpec {
    type Safety = crate::Unsafe;
}
