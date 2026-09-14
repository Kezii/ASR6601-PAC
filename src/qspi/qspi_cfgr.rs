#[doc = "Register `QSPI_CFGR` reader"]
pub type R = crate::R<QspiCfgrSpec>;
#[doc = "Register `QSPI_CFGR` writer"]
pub type W = crate::W<QspiCfgrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_cfgr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiCfgrSpec;
impl crate::RegisterSpec for QspiCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_cfgr::R`](R) reader structure"]
impl crate::Readable for QspiCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_cfgr::W`](W) writer structure"]
impl crate::Writable for QspiCfgrSpec {
    type Safety = crate::Unsafe;
}
