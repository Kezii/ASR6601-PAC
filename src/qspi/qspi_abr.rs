#[doc = "Register `QSPI_ABR` reader"]
pub type R = crate::R<QspiAbrSpec>;
#[doc = "Register `QSPI_ABR` writer"]
pub type W = crate::W<QspiAbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "alternate byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_abr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_abr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiAbrSpec;
impl crate::RegisterSpec for QspiAbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_abr::R`](R) reader structure"]
impl crate::Readable for QspiAbrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_abr::W`](W) writer structure"]
impl crate::Writable for QspiAbrSpec {
    type Safety = crate::Unsafe;
}
