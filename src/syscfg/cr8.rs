#[doc = "Register `CR8` reader"]
pub type R = crate::R<Cr8Spec>;
#[doc = "Register `CR8` writer"]
pub type W = crate::W<Cr8Spec>;
#[doc = "Field `QSPI_MEM_ENCRYPT_KEY` reader - encryption key for qspi memory"]
pub type QspiMemEncryptKeyR = crate::FieldReader<u32>;
#[doc = "Field `QSPI_MEM_ENCRYPT_KEY` writer - encryption key for qspi memory"]
pub type QspiMemEncryptKeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - encryption key for qspi memory"]
    #[inline(always)]
    pub fn qspi_mem_encrypt_key(&self) -> QspiMemEncryptKeyR {
        QspiMemEncryptKeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - encryption key for qspi memory"]
    #[inline(always)]
    pub fn qspi_mem_encrypt_key(&mut self) -> QspiMemEncryptKeyW<'_, Cr8Spec> {
        QspiMemEncryptKeyW::new(self, 0)
    }
}
#[doc = "control register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`cr8::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr8Spec;
impl crate::RegisterSpec for Cr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr8::R`](R) reader structure"]
impl crate::Readable for Cr8Spec {}
#[doc = "`write(|w| ..)` method takes [`cr8::W`](W) writer structure"]
impl crate::Writable for Cr8Spec {
    type Safety = crate::Unsafe;
}
