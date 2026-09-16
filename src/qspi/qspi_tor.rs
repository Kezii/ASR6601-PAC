#[doc = "Register `QSPI_TOR` reader"]
pub type R = crate::R<QspiTorSpec>;
#[doc = "Register `QSPI_TOR` writer"]
pub type W = crate::W<QspiTorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_tor::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_tor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiTorSpec;
impl crate::RegisterSpec for QspiTorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_tor::R`](R) reader structure"]
impl crate::Readable for QspiTorSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_tor::W`](W) writer structure"]
impl crate::Writable for QspiTorSpec {
    type Safety = crate::Unsafe;
}
