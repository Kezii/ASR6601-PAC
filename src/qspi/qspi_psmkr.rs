#[doc = "Register `QSPI_PSMKR` reader"]
pub type R = crate::R<QspiPsmkrSpec>;
#[doc = "Register `QSPI_PSMKR` writer"]
pub type W = crate::W<QspiPsmkrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "polling status mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_psmkr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiPsmkrSpec;
impl crate::RegisterSpec for QspiPsmkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_psmkr::R`](R) reader structure"]
impl crate::Readable for QspiPsmkrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_psmkr::W`](W) writer structure"]
impl crate::Writable for QspiPsmkrSpec {
    type Safety = crate::Unsafe;
}
