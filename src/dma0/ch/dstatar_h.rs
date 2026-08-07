#[doc = "Register `DSTATAR_H` reader"]
pub type R = crate::R<DstatarHSpec>;
#[doc = "Register `DSTATAR_H` writer"]
pub type W = crate::W<DstatarHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatar_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstatar_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstatarHSpec;
impl crate::RegisterSpec for DstatarHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstatar_h::R`](R) reader structure"]
impl crate::Readable for DstatarHSpec {}
#[doc = "`write(|w| ..)` method takes [`dstatar_h::W`](W) writer structure"]
impl crate::Writable for DstatarHSpec {
    type Safety = crate::Unsafe;
}
