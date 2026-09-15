#[doc = "Register `QSPI_CCR` reader"]
pub type R = crate::R<QspiCcrSpec>;
#[doc = "Register `QSPI_CCR` writer"]
pub type W = crate::W<QspiCcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "communication configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_ccr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiCcrSpec;
impl crate::RegisterSpec for QspiCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_ccr::R`](R) reader structure"]
impl crate::Readable for QspiCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_ccr::W`](W) writer structure"]
impl crate::Writable for QspiCcrSpec {
    type Safety = crate::Unsafe;
}
