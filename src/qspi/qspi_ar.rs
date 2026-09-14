#[doc = "Register `QSPI_AR` reader"]
pub type R = crate::R<QspiArSpec>;
#[doc = "Register `QSPI_AR` writer"]
pub type W = crate::W<QspiArSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "address register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_ar::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiArSpec;
impl crate::RegisterSpec for QspiArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_ar::R`](R) reader structure"]
impl crate::Readable for QspiArSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_ar::W`](W) writer structure"]
impl crate::Writable for QspiArSpec {
    type Safety = crate::Unsafe;
}
