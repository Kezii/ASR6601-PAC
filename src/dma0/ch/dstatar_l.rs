#[doc = "Register `DSTATAR_L` reader"]
pub type R = crate::R<DstatarLSpec>;
#[doc = "Register `DSTATAR_L` writer"]
pub type W = crate::W<DstatarLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatarLSpec;
impl crate::RegisterSpec for DstatarLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatar_l::R`](R) reader structure"]
impl crate::Readable for DstatarLSpec {}
#[doc = "`write(|w| ..)` method takes [`dstatar_l::W`](W) writer structure"]
impl crate::Writable for DstatarLSpec {
    type Safety = crate::Unsafe;
}
