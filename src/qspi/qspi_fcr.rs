#[doc = "Register `QSPI_FCR` reader"]
pub type R = crate::R<QspiFcrSpec>;
#[doc = "Register `QSPI_FCR` writer"]
pub type W = crate::W<QspiFcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_fcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiFcrSpec;
impl crate::RegisterSpec for QspiFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_fcr::R`](R) reader structure"]
impl crate::Readable for QspiFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_fcr::W`](W) writer structure"]
impl crate::Writable for QspiFcrSpec {
    type Safety = crate::Unsafe;
}
