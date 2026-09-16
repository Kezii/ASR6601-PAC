#[doc = "Register `QSPI_DCR` reader"]
pub type R = crate::R<QspiDcrSpec>;
#[doc = "Register `QSPI_DCR` writer"]
pub type W = crate::W<QspiDcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_dcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiDcrSpec;
impl crate::RegisterSpec for QspiDcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_dcr::R`](R) reader structure"]
impl crate::Readable for QspiDcrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_dcr::W`](W) writer structure"]
impl crate::Writable for QspiDcrSpec {
    type Safety = crate::Unsafe;
}
