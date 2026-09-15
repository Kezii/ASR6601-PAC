#[doc = "Register `QSPI_DLR` reader"]
pub type R = crate::R<QspiDlrSpec>;
#[doc = "Register `QSPI_DLR` writer"]
pub type W = crate::W<QspiDlrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dlr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiDlrSpec;
impl crate::RegisterSpec for QspiDlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_dlr::R`](R) reader structure"]
impl crate::Readable for QspiDlrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_dlr::W`](W) writer structure"]
impl crate::Writable for QspiDlrSpec {
    type Safety = crate::Unsafe;
}
