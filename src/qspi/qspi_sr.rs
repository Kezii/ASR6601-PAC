#[doc = "Register `QSPI_SR` reader"]
pub type R = crate::R<QspiSrSpec>;
#[doc = "Register `QSPI_SR` writer"]
pub type W = crate::W<QspiSrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiSrSpec;
impl crate::RegisterSpec for QspiSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_sr::R`](R) reader structure"]
impl crate::Readable for QspiSrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_sr::W`](W) writer structure"]
impl crate::Writable for QspiSrSpec {
    type Safety = crate::Unsafe;
}
