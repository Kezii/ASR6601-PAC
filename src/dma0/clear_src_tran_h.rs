#[doc = "Register `CLEAR_SRC_TRAN_H` reader"]
pub type R = crate::R<ClearSrcTranHSpec>;
#[doc = "Register `CLEAR_SRC_TRAN_H` writer"]
pub type W = crate::W<ClearSrcTranHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_src_tran_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_src_tran_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSrcTranHSpec;
impl crate::RegisterSpec for ClearSrcTranHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_src_tran_h::R`](R) reader structure"]
impl crate::Readable for ClearSrcTranHSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_src_tran_h::W`](W) writer structure"]
impl crate::Writable for ClearSrcTranHSpec {
    type Safety = crate::Unsafe;
}
