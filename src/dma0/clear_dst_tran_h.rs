#[doc = "Register `CLEAR_DST_TRAN_H` reader"]
pub type R = crate::R<ClearDstTranHSpec>;
#[doc = "Register `CLEAR_DST_TRAN_H` writer"]
pub type W = crate::W<ClearDstTranHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_dst_tran_h::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_dst_tran_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearDstTranHSpec;
impl crate::RegisterSpec for ClearDstTranHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_dst_tran_h::R`](R) reader structure"]
impl crate::Readable for ClearDstTranHSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_dst_tran_h::W`](W) writer structure"]
impl crate::Writable for ClearDstTranHSpec {
    type Safety = crate::Unsafe;
}
