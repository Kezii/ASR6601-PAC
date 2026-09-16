#[doc = "Register `QSPI_PIR` reader"]
pub type R = crate::R<QspiPirSpec>;
#[doc = "Register `QSPI_PIR` writer"]
pub type W = crate::W<QspiPirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "polling interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_pir::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiPirSpec;
impl crate::RegisterSpec for QspiPirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_pir::R`](R) reader structure"]
impl crate::Readable for QspiPirSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_pir::W`](W) writer structure"]
impl crate::Writable for QspiPirSpec {
    type Safety = crate::Unsafe;
}
