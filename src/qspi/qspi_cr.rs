#[doc = "Register `QSPI_CR` reader"]
pub type R = crate::R<QspiCrSpec>;
#[doc = "Register `QSPI_CR` writer"]
pub type W = crate::W<QspiCrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiCrSpec;
impl crate::RegisterSpec for QspiCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_cr::R`](R) reader structure"]
impl crate::Readable for QspiCrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_cr::W`](W) writer structure"]
impl crate::Writable for QspiCrSpec {
    type Safety = crate::Unsafe;
}
