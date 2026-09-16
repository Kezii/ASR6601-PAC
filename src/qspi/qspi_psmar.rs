#[doc = "Register `QSPI_PSMAR` reader"]
pub type R = crate::R<QspiPsmarSpec>;
#[doc = "Register `QSPI_PSMAR` writer"]
pub type W = crate::W<QspiPsmarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "polling status match register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_psmar::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiPsmarSpec;
impl crate::RegisterSpec for QspiPsmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_psmar::R`](R) reader structure"]
impl crate::Readable for QspiPsmarSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_psmar::W`](W) writer structure"]
impl crate::Writable for QspiPsmarSpec {
    type Safety = crate::Unsafe;
}
