#[doc = "Register `CLEAR_SRC_TRAN_L` reader"]
pub type R = crate::R<ClearSrcTranLSpec>;
#[doc = "Register `CLEAR_SRC_TRAN_L` writer"]
pub type W = crate::W<ClearSrcTranLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clear_src_tran_l::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_src_tran_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSrcTranLSpec;
impl crate::RegisterSpec for ClearSrcTranLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clear_src_tran_l::R`](R) reader structure"]
impl crate::Readable for ClearSrcTranLSpec {}
#[doc = "`write(|w| ..)` method takes [`clear_src_tran_l::W`](W) writer structure"]
impl crate::Writable for ClearSrcTranLSpec {
    type Safety = crate::Unsafe;
}
