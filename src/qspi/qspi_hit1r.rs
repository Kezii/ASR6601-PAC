#[doc = "Register `QSPI_HIT1R` reader"]
pub type R = crate::R<QspiHit1rSpec>;
#[doc = "Register `QSPI_HIT1R` writer"]
pub type W = crate::W<QspiHit1rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "hit1 times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_hit1r::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_hit1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiHit1rSpec;
impl crate::RegisterSpec for QspiHit1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_hit1r::R`](R) reader structure"]
impl crate::Readable for QspiHit1rSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_hit1r::W`](W) writer structure"]
impl crate::Writable for QspiHit1rSpec {
    type Safety = crate::Unsafe;
}
